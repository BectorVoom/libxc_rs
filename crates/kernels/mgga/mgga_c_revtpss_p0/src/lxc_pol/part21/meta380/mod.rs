//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1792;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1793;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1794;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1795;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1796;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta380<F: Float>(t1151: F, t3427: F, t3384: F, t1149: F, t3435: F, t3433: F, t1160: F, t3444: F, t1156: F, t3476: F, t1170: F, t12233: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12366: F, t12379: F, t12395: F, t12408: F, t3447: F, t3472: F, t3480: F, t435: F, t3475: F, t431: F, t426: F, t1168: F, t3453: F, t3479: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t12329: F, t12332: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12411, t12413, t12415, t12417, t12418, t12423) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1792::<F>(t1151, t3427, t3384, t1149, t3435, t3433, t1160, t3444, t1156, t3476);
        let t12426 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1793::<F>(t1170, t12233, t12240, t12242, t12245, t12251, t12360, t12363, t12366, t12379, t12395, t12408, t12413, t12417, t12418, t12423, t3447, t3472, t3480, t435);
        let (t12428, t12429) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1794::<F>(t3475, t431, t426);
        let t12430 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1795::<F>(t1168, t3453);
        let (t12431, t12448) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1796::<F>(t12430, t3479, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323, t12329, t12332);
    (t12411, t12413, t12415, t12417, t12418, t12423, t12426, t12428, t12429, t12430, t12431, t12448)
}
