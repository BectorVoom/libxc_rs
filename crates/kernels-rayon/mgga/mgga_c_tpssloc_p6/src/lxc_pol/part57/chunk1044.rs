//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1044/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1044(t128296: f64, t2039: f64, t33211: f64, t7801: f64, t28017: f64, t88: f64, t33596: f64, t28951: f64, t8601: f64, t102386: f64, t1873: f64, t122617: f64, t126127: f64, t126132: f64, t128371: f64, t128555: f64, t1458: f64, t24999: f64, t31532: f64, t33085: f64, t5493: f64) -> f64 {
    let t128953 = 4.0_f64 * t128296 * t2039;
    let t128955 = 4.0_f64 * t33211 * t7801;
    let t128956 = t88 * t28017;
    let t128958 = 2.0_f64 * t128956 * t2039;
    let t128960 = 4.0_f64 * t33596 * t7801;
    let t128962 = 2.0_f64 * t8601 * t28951;
    let t128968 = 2.0_f64 * t102386 * t1873;
    let t128970 = 4.0_f64 * t122617 * t1458 + 4.0_f64 * t126127 * t2039 + 2.0_f64 * t126132 * t2039 + 4.0_f64 * t24999 * t7801 + 2.0_f64 * t31532 * t5493 + 4.0_f64 * t33085 * t7801 + t128371 + 2.0_f64 * t128555 + t128953 + t128955 + t128958 + t128960 + t128962 + t128968;
    t128970
}
