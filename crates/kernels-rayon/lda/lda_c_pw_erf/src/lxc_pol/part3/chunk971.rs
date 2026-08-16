//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 971/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk971(t1: f64, t1742: f64, t1750: f64, t1755: f64, t1752: f64, t1753: f64, t279: f64, t2824: f64, t3117: f64, t3120: f64, t3124: f64, t3132: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11260 = t1742 * t1750 * t1 * t1755;
    let t11266 = 16.521134411652657_f64 * t1752 * t1753 * t2824 * t279;
    let t11272 = 192.98189186581325_f64 * t3117;
    let t11273 = 24.0_f64 * t3120;
    let t11274 = 24.0_f64 * t3124;
    let t11275 = 2069.0005882282467_f64 * t3132;
    (t11260, t11266, t11272, t11273, t11274, t11275)
}
