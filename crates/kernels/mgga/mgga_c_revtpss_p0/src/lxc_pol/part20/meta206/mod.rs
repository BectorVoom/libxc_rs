//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk978;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk979;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta206<F: Float>(t10227: F, t10228: F, t2349: F, t658: F, t2256: F, t9343: F, t100: F, t106: F, t107: F, t2358: F, t661: F, t2357: F, t2362: F, t108: F, t101: F, t10217: F, t105: F, t2344: F, t2351: F, t2354: F, t656: F, t659: F, t97: F, t114: F, t655: F, t10201: F, t10202: F, t10204: F, t10206: F, t10210: F, t10214: F, t69: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10229, t10232, t10233, t10236, t10237, t10241, t10243, t10246) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk978::<F>(t10227, t10228, t2349, t658, t2256, t9343, t100, t106, t107, t2358, t661, t2357);
        let (t10250, t10254) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk979::<F>(t10246, t2362, t10236, t108, t101, t10217, t10229, t10233, t10237, t10243, t105, t2344, t2351, t2354, t656, t659, t97);
        let (t10255, t10259) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk980::<F>(t114, t10254, t655, t10201, t10202, t10204, t10206, t10210, t10214, t69);
    (t10229, t10232, t10233, t10236, t10237, t10241, t10246, t10250, t10254, t10255, t10259)
}
