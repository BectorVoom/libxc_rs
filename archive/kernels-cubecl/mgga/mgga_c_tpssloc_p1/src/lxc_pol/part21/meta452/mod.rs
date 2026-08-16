//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2006;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2007;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta452<F: Float>(t11888: F, t11904: F, t11907: F, t11914: F, t1201: F, t1244: F, t1247: F, t15032: F, t15241: F, t15245: F, t15248: F, t15253: F, t15257: F, t15426: F, t15430: F, t15772: F, t15777: F, t1758: F, t3565: F, t3604: F, t3610: F, t3621: F, t3624: F, t3626: F, t470: F, t494: F, t5064: F, t5069: F, t5076: F, t5080: F, t5084: F, t5086: F, t15030: F, t1241: F, t1251: F, t5088: F, t3598: F, t1760: F, t3599: F, t11606: F, t225: F, t4941: F, t1751: F, t3481: F, t3630: F, t1238: F, t1252: F, t14972: F, t14980: F, t3487: F, t3593: F, t3600: F, t3631: F, t498: F, t5055: F, t5060: F, t5089: F) -> (F, F, F, F, F, F, F, F, F) {
        let t15785 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2006::<F>(t11888, t11904, t11907, t11914, t1201, t1244, t1247, t15032, t15241, t15245, t15248, t15253, t15257, t15426, t15430, t15772, t15777, t1758, t3565, t3604, t3610, t3621, t3624, t3626, t470, t494, t5064, t5069, t5076, t5080, t5084, t5086);
        let (t15786, t15787, t15789, t15790, t15794, t15797, t15800) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2007::<F>(t15030, t15785, t1241, t1251, t5088, t3598, t1760, t3599, t11606, t225, t4941, t1751, t3481);
        let (t15803, t15806) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2008::<F>(t1760, t3630, t3598, t1238, t1252, t14972, t14980, t15787, t15790, t15794, t15797, t15800, t3487, t3593, t3600, t3631, t498, t5055, t5060, t5089);
    (t15786, t15787, t15789, t15790, t15794, t15797, t15800, t15803, t15806)
}
