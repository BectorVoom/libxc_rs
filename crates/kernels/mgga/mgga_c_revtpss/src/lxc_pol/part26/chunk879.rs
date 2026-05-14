//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 879/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk879<F: Float>(t1175: F, t3495: F, t1188: F, t12487: F, t1189: F, t3515: F, t3523: F, t1187: F, t1170: F, t3471: F, t1168: F, t3479: F, t1156: F, t3451: F, t1169: F, t12430: F) -> (F, F, F, F, F, F, F, F) {
    let t12491 = t1175 * t3495;
    let t12494 = t12487 * t1188;
    let t12497 = t1189 * t3515;
    let t12500 = t3515 * t3523;
    let t12501 = t12500 * t1187;
    let t12504 = t1170 * t3471;
    let t12508 = t3471 * t3479 * t1168;
    let t12511 = t1156 * t3451;
    let t12514 = t12430 * t1169;
    (t12491, t12494, t12497, t12501, t12504, t12508, t12511, t12514)
}
