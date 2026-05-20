//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1106;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1107;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta323<F: Float>(t2777: F, t5759: F, t2439: F, t1398: F, t1892: F, t4086: F, t543: F, t2782: F, t5659: F, t72: F, t686: F, t4101: F, t136: F, t1883: F, t2457: F, t10139: F, t13926: F, t4100: F, t10014: F, t5741: F, t13790: F, t10022: F, t786: F, t4104: F, t2470: F, t5740: F, t1432: F, t5763: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14203, t14209, t14218) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1106::<F>(t2777, t5759, t2439, t1398, t1892, t4086, t543, t2782, t5659, t72, t686, t4101);
        let (t14221, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1107::<F>(t136, t1883, t2457, t10139, t13926, t543, t4100, t2782, t10014, t5741, t13790, t1398);
        let (t14233, t14239, t14241, t14243, t14252) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1108::<F>(t10022, t14230, t2782, t1892, t4086, t786, t4104, t2470, t5740, t4101, t1432, t5763);
    (t14203, t14209, t14218, t14221, t14227, t14229, t14233, t14239, t14241, t14243, t14252)
}
