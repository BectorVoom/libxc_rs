//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk921;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk922;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk923;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk924;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta193(t1763: f64, t3640: f64, t1254: f64, t1256: f64, t193: f64, t336: f64, t4700: f64, t4739: f64, t4742: f64, t4744: f64, t4747: f64, t4784: f64, t4788: f64, t4866: f64, t4868: f64, t4871: f64, t4873: f64, t4877: f64, t4881: f64, t4886: f64, t5091: f64, t28: f64, t265: f64, t504: f64, t4324: f64, t1081: f64, t1260: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t3966: f64, t4332: f64, t506: f64, t52: f64, t607: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t4712: f64, t671: f64, t88: f64, t1268: f64, t1458: f64, t2314: f64, t4026: f64, t4028: f64, t4072: f64, t1390: f64, t1845: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5095, t5098) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk921(t1763, t3640, t1254, t1256, t193, t336, t4700, t4739, t4742, t4744, t4747, t4784, t4788, t4866, t4868, t4871, t4873, t4877, t4881, t4886, t5091);
        let (t5099, t5106) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk922(t28, t265, t504, t4324, t5098, t1081, t1260, t1409, t1534, t1649, t1768, t3966, t4332, t506, t52, t607, t873, dens_threshold, rho1, zeta_threshold);
        let t5107 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk923(t4712, t5106);
        let t5113 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk924(t671, t88);
        let (t5118, t5122, t5126) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk925(t1268, t1458, t2314, t4026, t4028, t4072, t5113, t671, t1390, t1845, t193, t531);
    (t5095, t5099, t5107, t5113, t5118, t5122, t5126)
}
