//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1886;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1887;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta564<F: Float>(t25222: F, t4435: F, t14868: F, t2661: F, t93082: F, t14757: F, t25234: F, t14732: F, t25245: F, t14933: F, t2482: F, t25260: F, t814: F, t2689: F, t27239: F, t25277: F, t4458: F, t14685: F, t14756: F, t7021: F, t14760: F, t93015: F, t1955: F, t27198: F, t2769: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t99066, t99069, t99073, t99077, t99085) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1886::<F>(t25222, t4435, t14868, t2661, t93082, t14757, t25234, t14732, t25245, t14933, t2482, t25260, t814);
        let (t99091, t99099, t99102, t99113, t99191) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1887::<F>(t2689, t27239, t25277, t4458, t14685, t14756, t7021, t14760, t93015, t1955, t27198, t2769);
    (t99066, t99069, t99073, t99077, t99085, t99091, t99099, t99102, t99113, t99191)
}
