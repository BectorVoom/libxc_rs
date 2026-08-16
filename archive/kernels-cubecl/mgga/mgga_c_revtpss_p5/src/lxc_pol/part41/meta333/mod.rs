//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1129;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1130;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta333<F: Float>(t10726: F, t14868: F, t2661: F, t10868: F, t241: F, t820: F, t10811: F, t4452: F, t2719: F, t844: F, t4368: F, t2482: F, t814: F, t14671: F, t14686: F, t4366: F, t136: F, t1568: F, t2457: F, t2710: F, t2470: F, t4522: F, t874: F, t4469: F, t822: F, t4533: F, t72: F, t686: F, t2465: F, t1569: F, t867: F, t786: F, t2467: F, t122: F, t4480: F, t2466: F, t10995: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14871, t14894, t14907, t14925, t14931) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1129::<F>(t10726, t14868, t2661, t10868, t241, t820, t10811, t4452, t2719, t844, t4368, t2482, t814);
        let (t14934, t14948, t14951, t14972) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1130::<F>(t14671, t14686, t4366, t14931, t136, t1568, t2457, t2710, t2470, t4522, t874, t4469, t822);
        let (t14985, t14987, t14989, t14992) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1131::<F>(t4533, t72, t686, t2465, t1569, t867, t786, t2467, t122, t4480, t2466, t10995);
    (t14871, t14894, t14907, t14925, t14934, t14948, t14951, t14972, t14985, t14987, t14989, t14992)
}
