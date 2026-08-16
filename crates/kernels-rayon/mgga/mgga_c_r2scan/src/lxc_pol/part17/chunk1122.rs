//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1122/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1122(t11015: f64, t11568: f64, t3434: f64, t10680: f64, t10681: f64, t10683: f64, t2482: f64, t10673: f64, t10674: f64, t10676: f64, t104: f64, t920: f64) -> (f64, f64, f64, f64) {
    let t40334 = t3434 * t11015 * t11568;
    let t40341 = t10680 * t10681 * t2482 * t10683;
    let t40345 = t10673 * t10674 * t2482 * t10676;
    let t40393 = t104 * t920;
    (t40334, t40341, t40345, t40393)
}
