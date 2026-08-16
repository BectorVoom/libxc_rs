//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 988/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk988(t46671: f64, t2375: f64, t37579: f64, t13296: f64, t1339: f64, t13429: f64, t1457: f64, t1537: f64, t1562: f64, t1572: f64, t37551: f64, t42256: f64, t42259: f64, t44396: f64, t44564: f64, t4614: f64, t46626: f64, t46630: f64, t46633: f64, t46635: f64, t46638: f64, t46642: f64, t46645: f64, t46646: f64, t46654: f64, t46658: f64, t46662: f64, t46668: f64, t536: f64, t549: f64, t590: f64) -> f64 {
    let t46672 = 0.23005755572352449806e1_f64 * t46671;
    let t46674 = 0.27805936629216998521e0_f64 * t37579 * t2375;
    let t46680 = 0.14300195980740170668e1_f64 * t1572 * t1457 * t44564 + 0.35750489951850426669e0_f64 * t536 * t46626 + t46630 + t46633 - t46635 - t46638 + t46642 - t46645 - 0.38342925953920749677e0_f64 * t46646 - 0.51123901271894332902e0_f64 * t1537 * t1339 * t13296 * t590 - t46654 - t46658 - t46662 - 0.79445533226334281487e-1_f64 * t37551 * t549 * t44396 - t46668 + t46672 + t46674 + 0.63904876589867916128e-1_f64 * t42256 + 0.59584149919750711116e-1_f64 * t42259 - 0.18404604457881959845e2_f64 * t1562 * t4614 * t13429;
    t46680
}
