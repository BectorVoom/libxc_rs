//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 625/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk625<F: Float>(t482: F, t6628: F, t3604: F, t1042: F, t3611: F, t1469: F, t3628: F, t5351: F, t3626: F, t6587: F, t371: F, t372: F, t1235: F, t1247: F, t1791: F, t1797: F, t3600: F, t3610: F, t3625: F, t3671: F, t3711: F, t484: F, t5254: F, t5256: F, t5266: F, t5274: F, t5293: F, t5323: F, t5327: F, t6595: F, t6598: F, t6602: F, t6611: F, t6619: F, t6625: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6629 = t482 * t6628;
    let t6630 = t6629 * t3604;
    let t6631 = t1042 * t6630;
    let t6634 = t6629 * t3611;
    let t6635 = t1042 * t6634;
    let t6638 = t3628 * t1469;
    let t6639 = t5351 * t6638;
    let t6640 = t3626 * t6639;
    let t6645 = t482 * t6587;
    let t6647 = t371 * t372 * t6645;
    let t6651 = 0.72409452821628889107e-2 * t6595 * t484 - 0.22866142996303859718e-2 * t6598 * t484 + 0.21437009059034868486e-3 * t6602 * t484 - 0.22866142996303859718e-2 * t5293 * t1797 - 0.15244095330869239812e-2 * t5254 + 0.28582678745379824648e-3 * t5256 + 0.42874018118069736972e-3 * t3671 * t6611 + 0.22866142996303859718e-2 * t5323 * t1791 + 0.42874018118069736972e-3 * t5274 * t1797 + 0.28582678745379824648e-3 * t3711 * t6619 + 0.21437009059034868486e-3 * t1247 * t6625 + 0.42874018118069736972e-3 * t3600 * t6631 - 0.21437009059034868486e-3 * t3610 * t6635 - 0.28582678745379824648e-3 * t3625 * t6640 - 0.42874018118069736972e-3 * t5327 * t1791 - 0.21437009059034868486e-3 * t1235 * t6647 + 0.28582678745379824648e-3 * t5266;
    (t6630, t6631, t6634, t6635, t6638, t6639, t6640, t6645, t6647, t6651)
}
