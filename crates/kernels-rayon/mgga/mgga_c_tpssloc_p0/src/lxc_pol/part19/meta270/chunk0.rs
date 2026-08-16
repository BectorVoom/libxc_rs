//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1026/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1026(t11918: f64, t1241: f64, t11868: f64, t466: f64, t225: f64, t3591: f64, t3482: f64, t1190: f64, t3590: f64, t1251: f64, t3630: f64, t3598: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11919 = t1241 * t11918;
    let t11923 = t466 * t11868;
    let t11925 = t3591 * t225;
    let t11928 = t3482 * t225;
    let t11931 = t1190 * t3590;
    let t11934 = t1251 * t3630;
    let t11935 = t3598 * t11934;
    (t11919, t11923, t11925, t11928, t11931, t11935)
}
