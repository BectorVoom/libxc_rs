//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1129/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1129(t1054: f64, t6583: f64, t7326: f64, t10799: f64, t2207: f64, t3613: f64, t10814: f64, t2651: f64, t10698: f64, t2593: f64, t11805: f64, t37641: f64) -> (f64, f64, f64, f64, f64) {
    let t39664 = t6583 * t1054 * t7326;
    let t39667 = t2207 * t3613 * t10799;
    let t39669 = t2651 * t10814;
    let t39672 = t10698 * t2593;
    let t39674 = t37641 * t11805;
    (t39664, t39667, t39669, t39672, t39674)
}
