//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1655;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1656;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1657;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta430<F: Float>(t19676: F, t19679: F, t19688: F, t19699: F, t225: F, t1819: F, t68: F, t1995: F, t6330: F, t1307: F, t5187: F, t5279: F, t1365: F, t6347: F, t1347: F, t19631: F, t1345: F, t1348: F, t1821: F, t5272: F, t5278: F, t5280: F, t5283: F, t546: F, t548: F, t6404: F, t6408: F, t6411: F, t550: F, t1380: F, t3792: F, t5286: F, t5335: F, t1824: F, t1834: F, t5250: F, t562: F, t6387: F, t12250: F, t1351: F, t5287: F, t5348: F, t1336: F, t16047: F, t19654: F, t19658: F, t19661: F, t19668: F, t19674: F, t3777: F, t5234: F, t5334: F, t5336: F, t5349: F, t6448: F, t6451: F, t6454: F, t6456: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19702, t19708, t19716, t19719) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1655::<F>(t19676, t19679, t19688, t19699, t225, t1819, t68, t1995, t6330, t1307, t5187, t5279);
        let t19731 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1656::<F>(t1365, t6347, t1307, t1347, t19631, t1345, t1348, t1819, t1821, t19702, t19708, t19716, t19719, t5272, t5278, t5280, t5283, t546, t548, t6404, t6408, t6411);
        let (t19732, t19733, t19735, t19736, t19739, t19740, t19743) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1657::<F>(t19731, t550, t1380, t3792, t5286, t5335, t1824, t1834, t5250, t562, t6387);
        let (t19744, t19745, t19748, t19755) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1658::<F>(t12250, t1351, t19743, t5250, t5287, t5348, t1336, t16047, t19654, t19658, t19661, t19668, t19674, t19733, t19736, t19740, t3777, t5234, t5334, t5336, t5349, t6448, t6451, t6454, t6456);
    (t19731, t19732, t19735, t19736, t19739, t19740, t19743, t19744, t19745, t19748, t19755)
}
