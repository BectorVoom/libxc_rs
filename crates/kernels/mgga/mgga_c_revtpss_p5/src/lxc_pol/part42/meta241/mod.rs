//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk922;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk923;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta241<F: Float>(t1169: F, t6502: F, t3479: F, t6486: F, t3483: F, t5044: F, t6423: F, t6427: F, t6431: F, t448: F, t1756: F, t1188: F, t3503: F, t3510: F, t5093: F, t6443: F, t6450: F, t6456: F, t6458: F, t6462: F, t6465: F, t6468: F, t3523: F, t1161: F, t1180: F, t1745: F, t1757: F, t3452: F, t3477: F, t3496: F, t3521: F, t435: F, t5120: F, t5158: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6481: F, t6487: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6503, t6506, t6513, t6514, t6518) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk922::<F>(t1169, t6502, t3479, t6486, t3483, t5044, t6423, t6427, t6431, t448, t1756);
        let (t6519, t6534) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk923::<F>(t1188, t6518, t3503, t3510, t5044, t5093, t6423, t6427, t6431, t6443, t6450, t6456, t6458, t6462, t6465, t6468);
        let (t6535, t6538, t6541) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk924::<F>(t1188, t6534, t3523, t6518, t1161, t1180, t1745, t1757, t3452, t3477, t3496, t3521, t435, t5120, t5158, t6435, t6437, t6441, t6473, t6476, t6481, t6487, t6503, t6506, t6514, t6519);
    (t6503, t6506, t6513, t6514, t6518, t6519, t6534, t6535, t6538, t6541)
}
