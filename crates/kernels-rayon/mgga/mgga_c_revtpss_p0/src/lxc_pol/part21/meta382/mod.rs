//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1802;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1803;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1804;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta382(t1187: f64, t3497: f64, t3523: f64, t1175: f64, t3495: f64, t1188: f64, t1189: f64, t3515: f64, t1170: f64, t3471: f64, t1168: f64, t3479: f64, t1156: f64, t3451: f64, t1169: f64, t12430: f64, t12252: f64, t12259: f64, t12261: f64, t12263: f64, t12265: f64, t12271: f64, t12275: f64, t12279: f64, t12284: f64, t12289: f64, t12292: f64, t12323: f64, t12329: f64, t12332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t12487 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1802(t1187, t3497);
        let (t12488, t12491, t12494, t12497, t12500, t12501, t12504, t12508) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1803(t12487, t3523, t1175, t3495, t1188, t1189, t3515, t1187, t1170, t3471, t1168, t3479);
        let (t12511, t12514, t12531) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1804(t1156, t3451, t1169, t12430, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323, t12329, t12332);
    (t12487, t12488, t12491, t12494, t12497, t12500, t12501, t12504, t12508, t12511, t12514, t12531)
}
