//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1840;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1841;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta491(t7284: f64, t974: f64, t24847: f64, t1089: f64, t491: f64, t7327: f64, t15707: f64, t7376: f64, t24574: f64, t7365: f64, t1235: f64, t477: f64, t1090: f64, t7362: f64, t24837: f64, t3612: f64, t1244: f64, t2121: f64, t24804: f64, t24807: f64, t24812: f64, t24817: f64, t24823: f64, t24827: f64, t24830: f64, t24834: f64, t24838: f64, t24841: f64, t24845: f64, t3610: f64, t3624: f64, t7283: f64, t7373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24848, t24849) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1840(t7284, t974, t24847);
        let (t24850, t24851, t24852, t24853, t24856, t24858, t24859, t24860, t24863) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1841(t1089, t491, t7327, t15707, t7376, t24574, t7365, t1235, t477, t1090, t7362, t24837, t3612);
        let t24866 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1842(t1244, t2121, t24804, t24807, t24812, t24817, t24823, t24827, t24830, t24834, t24838, t24841, t24845, t24849, t24853, t24856, t24860, t24863, t3610, t3624, t7283, t7373);
    (t24848, t24849, t24850, t24851, t24852, t24853, t24856, t24858, t24859, t24860, t24863, t24866)
}
