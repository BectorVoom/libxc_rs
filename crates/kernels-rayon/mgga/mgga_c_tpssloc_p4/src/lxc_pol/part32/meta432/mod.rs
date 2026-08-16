//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1662;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1663;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1664;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta432(t19676: f64, t19679: f64, t19688: f64, t19699: f64, t225: f64, t1819: f64, t68: f64, t1995: f64, t6330: f64, t1307: f64, t5187: f64, t5279: f64, t1365: f64, t6347: f64, t1347: f64, t19631: f64, t1345: f64, t1348: f64, t1821: f64, t5272: f64, t5278: f64, t5280: f64, t5283: f64, t546: f64, t548: f64, t6404: f64, t6408: f64, t6411: f64, t550: f64, t1380: f64, t3792: f64, t5286: f64, t5335: f64, t1824: f64, t1834: f64, t5250: f64, t562: f64, t6387: f64, t12250: f64, t1351: f64, t5287: f64, t5348: f64, t1336: f64, t16047: f64, t19654: f64, t19658: f64, t19661: f64, t19668: f64, t19674: f64, t3777: f64, t5234: f64, t5334: f64, t5336: f64, t5349: f64, t6448: f64, t6451: f64, t6454: f64, t6456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19702, t19708, t19716, t19719) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1662(t19676, t19679, t19688, t19699, t225, t1819, t68, t1995, t6330, t1307, t5187, t5279);
        let t19731 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1663(t1365, t6347, t1307, t1347, t19631, t1345, t1348, t1819, t1821, t19702, t19708, t19716, t19719, t5272, t5278, t5280, t5283, t546, t548, t6404, t6408, t6411);
        let (t19732, t19733, t19735, t19736, t19739, t19740, t19743) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1664(t19731, t550, t1380, t3792, t5286, t5335, t1824, t1834, t5250, t562, t6387);
        let (t19744, t19745, t19748, t19755) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1665(t12250, t1351, t19743, t5250, t5287, t5348, t1336, t16047, t19654, t19658, t19661, t19668, t19674, t19733, t19736, t19740, t3777, t5234, t5334, t5336, t5349, t6448, t6451, t6454, t6456);
    (t19731, t19732, t19735, t19736, t19739, t19740, t19743, t19744, t19745, t19748, t19755)
}
