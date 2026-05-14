//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1263/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1263<F: Float>(t140: F, t6658: F, t1222: F, t6662: F, t1774: F, t5284: F, t1250: F, t3720: F, t1266: F, t17629: F, t21228: F, t21234: F, t21236: F, t21239: F, t21242: F, t21246: F, t21249: F, t3625: F, t3718: F, t5381: F, t5384: F, t5397: F) -> (F, F) {
    let t21251 = t140 * t6658;
    let t21252 = t1222 * t21251;
    let t21254 = t140 * t6662;
    let t21255 = t1222 * t21254;
    let t21257 = t1774 * t5284;
    let t21258 = t21257 * t1250;
    let t21259 = t3720 * t21258;
    let t21264 = -0.28582678745379824648e-3 * t3625 * t21228 + t17629 / 648.0 + 0.15879265969655458138e-3 * t21234 + t1222 * t21236 / 108.0 + t1222 * t21239 / 36.0 + 0.15244095330869239812e-2 * t21242 * t1266 + 0.42874018118069736972e-3 * t5384 * t21246 + t21249 / 162.0 - t21252 / 864.0 - t21255 / 432.0 - 0.42874018118069736972e-3 * t3718 * t21259 - 0.28582678745379824648e-3 * t5381 * t5397;
    (t21257, t21264)
}
