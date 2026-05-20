//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1222;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1223;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta356<F: Float>(t10577: F, t10582: F, t10584: F, t10586: F, t14385: F, t14388: F, t14392: F, t14396: F, t14428: F, t14433: F, t14434: F, t9514: F, t9517: F, t9521: F, t9524: F, t1531: F, t37: F, t2612: F, t4392: F, t72: F, t757: F, t14425: F, t150: F, t190: F, t10608: F, t2258: F, t4402: F, t4401: F, t2414: F, t4311: F, t10428: F, t1522: F, t10613: F, t10592: F, t10596: F, t10604: F, t10611: F, t14442: F, t14443: F, t14444: F, t9542: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t14612 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1222::<F>(t10577, t10582, t10584, t10586, t14385, t14388, t14392, t14396, t14428, t14433, t14434, t9514, t9517, t9521, t9524);
        let (t14615, t14618, t14620, t14621, t14622) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1223::<F>(t1531, t37, t2612, t4392, t72, t757, t14425, t150, t190, t10608, t2258, t4402);
        let (t14624, t14626, t14628, t14629, t14630) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1224::<F>(t14622, t4401, t2414, t4311, t10428, t1522, t10613, t10592, t10596, t10604, t10611, t14442, t14443, t14444, t14615, t14618, t14620, t14621, t9542);
    (t14612, t14615, t14618, t14620, t14621, t14624, t14626, t14628, t14629, t14630)
}
