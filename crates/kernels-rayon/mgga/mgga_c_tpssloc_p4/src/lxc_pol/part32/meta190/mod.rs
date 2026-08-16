//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk928;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk929;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk930;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk931;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk932;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta190(t1755: f64, t5079: f64, t1215: f64, t1751: f64, t1246: f64, t493: f64, t5052: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1756: f64, t1758: f64, t3604: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t4964: f64, t5064: f64, t5069: f64, t5073: f64, t5076: f64, t1241: f64, t1238: f64, t1252: f64, t1761: f64, t3487: f64, t3593: f64, t4941: f64, t4943: f64, t4945: f64, t4947: f64, t498: f64, t5053: f64, t5055: f64, t5060: f64, t1763: f64, t3640: f64, t1254: f64, t1256: f64, t193: f64, t336: f64, t4700: f64, t4739: f64, t4742: f64, t4744: f64, t4747: f64, t4784: f64, t4788: f64, t4866: f64, t4868: f64, t4871: f64, t4873: f64, t4877: f64, t4881: f64, t4886: f64, t28: f64, t265: f64, t504: f64, t4324: f64, t1081: f64, t1260: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t3966: f64, t4332: f64, t506: f64, t52: f64, t607: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t4712: f64, t671: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5080, t5083, t5084, t5086, t5088) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk928(t1755, t5079, t1215, t1751, t1246, t493, t5052, t1201, t1244, t1247, t1249, t1729, t1756, t1758, t3604, t3610, t3624, t470, t494, t4964, t5064, t5069, t5073, t5076);
        let (t5089, t5091) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk929(t1241, t5088, t1238, t1252, t1761, t3487, t3593, t4941, t4943, t4945, t4947, t498, t5053, t5055, t5060);
        let (t5095, t5098) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk930(t1763, t3640, t1254, t1256, t193, t336, t4700, t4739, t4742, t4744, t4747, t4784, t4788, t4866, t4868, t4871, t4873, t4877, t4881, t4886, t5091);
        let (t5099, t5106) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk931(t28, t265, t504, t4324, t5098, t1081, t1260, t1409, t1534, t1649, t1768, t3966, t4332, t506, t52, t607, t873, dens_threshold, rho1, zeta_threshold);
        let t5107 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk932(t4712, t5106);
        let t5113 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk933(t671, t88);
    (t5080, t5083, t5084, t5086, t5088, t5089, t5091, t5095, t5099, t5107, t5113)
}
