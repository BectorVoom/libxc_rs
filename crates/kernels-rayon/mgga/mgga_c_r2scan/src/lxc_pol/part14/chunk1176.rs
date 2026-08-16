//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1176/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1176(t32212: f64, t481: f64, t11550: f64, t792: f64, t10648: f64, t10971: f64, t11564: f64, t23987: f64, t795: f64, t10966: f64, t1103: f64, t269: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40631 = t32212 * t481;
    let t40635 = t11550 * t792;
    let t40642 = t10648 * t10971 * t11564;
    let t40644 = t11550 * t481;
    let t40652 = t23987 * t795;
    let t40659 = t10966 * t1103 * t955 * t269;
    (t40631, t40635, t40642, t40644, t40652, t40659)
}
