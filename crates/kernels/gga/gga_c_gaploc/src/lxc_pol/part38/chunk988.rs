//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 988/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk988<F: Float>(t46671: F, t2375: F, t37579: F, t13296: F, t1339: F, t13429: F, t1457: F, t1537: F, t1562: F, t1572: F, t37551: F, t42256: F, t42259: F, t44396: F, t44564: F, t4614: F, t46626: F, t46630: F, t46633: F, t46635: F, t46638: F, t46642: F, t46645: F, t46646: F, t46654: F, t46658: F, t46662: F, t46668: F, t536: F, t549: F, t590: F) -> F {
    let t46672 = F::new(0.23005755572352449806e1) * t46671;
    let t46674 = F::new(0.27805936629216998521e0) * t37579 * t2375;
    let t46680 = F::new(0.14300195980740170668e1) * t1572 * t1457 * t44564 + F::new(0.35750489951850426669e0) * t536 * t46626 + t46630 + t46633 - t46635 - t46638 + t46642 - t46645 - F::new(0.38342925953920749677e0) * t46646 - F::new(0.51123901271894332902e0) * t1537 * t1339 * t13296 * t590 - t46654 - t46658 - t46662 - F::new(0.79445533226334281487e-1) * t37551 * t549 * t44396 - t46668 + t46672 + t46674 + F::new(0.63904876589867916128e-1) * t42256 + F::new(0.59584149919750711116e-1) * t42259 - F::new(0.18404604457881959845e2) * t1562 * t4614 * t13429;
    t46680
}
