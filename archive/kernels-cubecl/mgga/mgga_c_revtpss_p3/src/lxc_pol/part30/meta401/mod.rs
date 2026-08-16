//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1506;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1507;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta401<F: Float>(t14676: F, t4364: F, t837: F, t2646: F, t4365: F, t136: F, t243: F, t220: F, t14671: F, t10777: F, t125: F, t4343: F, t2747: F, t4450: F, t10779: F, t1548: F, t10811: F, t4447: F, t2749: F, t10673: F, t10676: F, t14668: F, t14675: F, t2745: F, t4362: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14678, t14682, t14685, t14686, t14688, t14690, t14691) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1506::<F>(t14676, t4364, t837, t2646, t4365, t136, t243, t220, t14671, t10777, t125, t4343);
        let (t14693, t14697, t14701, t14703, t14705, t14707) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1507::<F>(t14691, t2747, t837, t2646, t4450, t10779, t1548, t10777, t10811, t4447, t14676, t2749);
        let t14711 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1508::<F>(t10673, t10676, t14668, t14675, t14678, t14682, t14690, t14693, t14697, t14703, t14705, t14707, t2745, t4362);
    (t14678, t14682, t14685, t14686, t14688, t14693, t14697, t14701, t14707, t14711)
}
