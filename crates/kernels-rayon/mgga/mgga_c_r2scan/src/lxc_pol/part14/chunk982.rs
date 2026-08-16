//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 982/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk982(t2847: f64, t797: f64, t2526: f64, t2333: f64, t983: f64, t795: f64, t2867: f64, t792: f64, t158: f64, t955: f64, t874: f64, t3446: f64, t3447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11531 = t797 * t2847;
    let t11550 = t797 * t2526;
    let t11554 = t2333 * t983;
    let t11555 = t11554 * t795;
    let t11559 = t2867 * t792;
    let t11563 = t158 * t955;
    let t11564 = t11563 * t874;
    let t11566 = t3446 * t3447 * t11564;
    (t11531, t11550, t11554, t11555, t11559, t11563, t11564, t11566)
}
