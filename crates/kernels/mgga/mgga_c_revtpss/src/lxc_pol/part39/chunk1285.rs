//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1285/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1285<F: Float>(t3368: F, t5277: F, t1042: F, t3704: F, t5274: F, t1774: F, t3588: F, t1250: F, t3720: F, t1285: F, t17395: F, t1032: F, t5216: F, t1246: F, t1252: F, t12956: F, t12999: F, t13012: F, t13015: F, t13018: F, t3631: F, t3647: F, t3711: F, t3718: F, t5279: F, t5304: F) -> (F, F) {
    let t17588 = t5277 * t3368;
    let t17589 = t1042 * t17588;
    let t17593 = 0.28582678745379824648e-3 * t5274 * t3704;
    let t17600 = t1774 * t3588;
    let t17601 = t17600 * t1250;
    let t17602 = t3720 * t17601;
    let t17605 = t1285 * t17395;
    let t17608 = t5216 * t1032;
    let t17609 = t17608 * t1246;
    let t17614 = 0.28582678745379824648e-3 * t3711 * t17589 + t17593 + 0.28582678745379824648e-3 * t12956 * t5279 - t12999 / 432.0 + t13012 / 648.0 - t13015 / 864.0 + t13018 / 648.0 - 0.21437009059034868486e-3 * t3718 * t17602 + 0.15244095330869239812e-2 * t17605 * t3631 + 0.42874018118069736972e-3 * t17609 * t1252 + 0.47637797908966374414e-3 * t3647 * t5304;
    (t17600, t17614)
}
