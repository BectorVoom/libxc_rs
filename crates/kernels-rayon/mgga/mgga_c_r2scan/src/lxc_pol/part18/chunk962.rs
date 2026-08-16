//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 962/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk962(t10935: f64, t3446: f64, t970: f64, t58: f64, t897: f64, t597: f64, t10649: f64, t10648: f64, t10681: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11580 = t3446 * t10935 * t970;
    let t11582 = t58 * t897;
    let t11583 = t11582 * t597;
    let t11584 = t10649 * t11583;
    let t11585 = t10648 * t11584;
    let t11587 = t10681 * t897;
    (t11580, t11582, t11583, t11584, t11585, t11587)
}
