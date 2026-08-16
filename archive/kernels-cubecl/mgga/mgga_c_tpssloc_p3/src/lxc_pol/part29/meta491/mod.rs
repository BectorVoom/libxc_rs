//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1840;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1841;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta491<F: Float>(t7284: F, t974: F, t24847: F, t1089: F, t491: F, t7327: F, t15707: F, t7376: F, t24574: F, t7365: F, t1235: F, t477: F, t1090: F, t7362: F, t24837: F, t3612: F, t1244: F, t2121: F, t24804: F, t24807: F, t24812: F, t24817: F, t24823: F, t24827: F, t24830: F, t24834: F, t24838: F, t24841: F, t24845: F, t3610: F, t3624: F, t7283: F, t7373: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24848, t24849) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1840::<F>(t7284, t974, t24847);
        let (t24850, t24851, t24852, t24853, t24856, t24858, t24859, t24860, t24863) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1841::<F>(t1089, t491, t7327, t15707, t7376, t24574, t7365, t1235, t477, t1090, t7362, t24837, t3612);
        let t24866 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1842::<F>(t1244, t2121, t24804, t24807, t24812, t24817, t24823, t24827, t24830, t24834, t24838, t24841, t24845, t24849, t24853, t24856, t24860, t24863, t3610, t3624, t7283, t7373);
    (t24848, t24849, t24850, t24851, t24852, t24853, t24856, t24858, t24859, t24860, t24863, t24866)
}
