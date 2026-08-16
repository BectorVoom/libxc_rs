//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 505/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk505(t2104: f64, t813: f64, t1284: f64, t544: f64, t795: f64, t511: f64, t808: f64, t181: f64, t494: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2106 = 4.0_f64 / 15.0_f64 * t2104 * t813;
    let t2108 = 4.0_f64 / 15.0_f64 * t1284 * t813;
    let t2110 = 2.0_f64 / 15.0_f64 * t795 * t544;
    let t2112 = 2.0_f64 / 15.0_f64 * t511 * t808;
    let t2113 = t494 * t181;
    let t2114 = t2113 * t184;
    (t2106, t2108, t2110, t2112, t2113, t2114)
}
