//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1802;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1803;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1804;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta382<F: Float>(t1187: F, t3497: F, t3523: F, t1175: F, t3495: F, t1188: F, t1189: F, t3515: F, t1170: F, t3471: F, t1168: F, t3479: F, t1156: F, t3451: F, t1169: F, t12430: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t12329: F, t12332: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t12487 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1802::<F>(t1187, t3497);
        let (t12488, t12491, t12494, t12497, t12500, t12501, t12504, t12508) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1803::<F>(t12487, t3523, t1175, t3495, t1188, t1189, t3515, t1187, t1170, t3471, t1168, t3479);
        let (t12511, t12514, t12531) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1804::<F>(t1156, t3451, t1169, t12430, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323, t12329, t12332);
    (t12487, t12488, t12491, t12494, t12497, t12500, t12501, t12504, t12508, t12511, t12514, t12531)
}
