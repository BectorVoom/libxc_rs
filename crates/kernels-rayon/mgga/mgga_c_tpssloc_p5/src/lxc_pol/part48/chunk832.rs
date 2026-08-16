//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 832/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk832(t1385: f64, t31090: f64, t22635: f64, t1992: f64, t1377: f64, t2015: f64, t1307: f64, t22633: f64, t794: f64, t8454: f64, t6897: f64, t225: f64, t567: f64, t6955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31091 = t31090 * t1385;
    let t31092 = t22635 * t31091;
    let t31094 = 0.3289868133696452873e-1_f64 * t1992 * t31092;
    let t31099 = t1377 * t2015;
    let t31100 = t31099 * t1307;
    let t31101 = t22635 * t31100;
    let t31103 = 0.3289868133696452873e-1_f64 * t22633 * t31101;
    let t31104 = t794 * t8454;
    let t31106 = 0.82246703342411321825e-2_f64 * t6897 * t31104;
    let t31108 = t6955 * t225 * t567;
    (t31091, t31092, t31094, t31099, t31100, t31101, t31103, t31104, t31106, t31108)
}
