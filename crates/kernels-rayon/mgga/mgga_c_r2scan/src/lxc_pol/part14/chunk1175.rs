//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1175/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1175(t40296: f64, t792: f64, t10935: f64, t2813: f64, t3446: f64, t6897: f64, t983: f64, t2330: f64, t1563: f64, t3574: f64, t3261: f64, t498: f64, t97: f64) -> (f64, f64, f64, f64, f64) {
    let t40595 = t40296 * t792;
    let t40603 = t3446 * t10935 * t2813;
    let t40608 = t6897 * t983;
    let t40609 = t40608 * t2330;
    let t40620 = t3574 * t1563;
    let t40630 = t97 * t3261 * t498;
    (t40595, t40603, t40609, t40620, t40630)
}
