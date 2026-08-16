//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1792;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1793;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1794;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1795;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1796;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta380(t1151: f64, t3427: f64, t3384: f64, t1149: f64, t3435: f64, t3433: f64, t1160: f64, t3444: f64, t1156: f64, t3476: f64, t1170: f64, t12233: f64, t12240: f64, t12242: f64, t12245: f64, t12251: f64, t12360: f64, t12363: f64, t12366: f64, t12379: f64, t12395: f64, t12408: f64, t3447: f64, t3472: f64, t3480: f64, t435: f64, t3475: f64, t431: f64, t426: f64, t1168: f64, t3453: f64, t3479: f64, t12252: f64, t12259: f64, t12261: f64, t12263: f64, t12265: f64, t12271: f64, t12275: f64, t12279: f64, t12284: f64, t12289: f64, t12292: f64, t12323: f64, t12329: f64, t12332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12411, t12413, t12415, t12417, t12418, t12423) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1792(t1151, t3427, t3384, t1149, t3435, t3433, t1160, t3444, t1156, t3476);
        let t12426 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1793(t1170, t12233, t12240, t12242, t12245, t12251, t12360, t12363, t12366, t12379, t12395, t12408, t12413, t12417, t12418, t12423, t3447, t3472, t3480, t435);
        let (t12428, t12429) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1794(t3475, t431, t426);
        let t12430 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1795(t1168, t3453);
        let (t12431, t12448) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1796(t12430, t3479, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323, t12329, t12332);
    (t12411, t12413, t12415, t12417, t12418, t12423, t12426, t12428, t12429, t12430, t12431, t12448)
}
