//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 983/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk983(t609: f64, t18104: f64, t4440: f64, t2104: f64, t3754: f64, t2642: f64, t12617: f64, t17960: f64, t1608: f64, t286: f64, t1610: f64, t833: f64, t6171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t614 = 0.0_f64 < t609;
    let t18105 = t4440 * t18104;
    let t18108 = t2104 * t3754;
    let t18109 = t18108 * t2642;
    let t18110 = t12617 * t18109;
    let t18114 = piecewise3(t614, t17960, -t17960);
    let t18115 = t1608 * t18114;
    let t18116 = t286 * t18115;
    let t18119 = t833 * t1610;
    let t18120 = t6171 * t18119;
    (t18105, t18110, t18114, t18116, t18119, t18120)
}
