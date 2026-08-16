//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2006;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2007;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta452(t11888: f64, t11904: f64, t11907: f64, t11914: f64, t1201: f64, t1244: f64, t1247: f64, t15032: f64, t15241: f64, t15245: f64, t15248: f64, t15253: f64, t15257: f64, t15426: f64, t15430: f64, t15772: f64, t15777: f64, t1758: f64, t3565: f64, t3604: f64, t3610: f64, t3621: f64, t3624: f64, t3626: f64, t470: f64, t494: f64, t5064: f64, t5069: f64, t5076: f64, t5080: f64, t5084: f64, t5086: f64, t15030: f64, t1241: f64, t1251: f64, t5088: f64, t3598: f64, t1760: f64, t3599: f64, t11606: f64, t225: f64, t4941: f64, t1751: f64, t3481: f64, t3630: f64, t1238: f64, t1252: f64, t14972: f64, t14980: f64, t3487: f64, t3593: f64, t3600: f64, t3631: f64, t498: f64, t5055: f64, t5060: f64, t5089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t15785 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2006(t11888, t11904, t11907, t11914, t1201, t1244, t1247, t15032, t15241, t15245, t15248, t15253, t15257, t15426, t15430, t15772, t15777, t1758, t3565, t3604, t3610, t3621, t3624, t3626, t470, t494, t5064, t5069, t5076, t5080, t5084, t5086);
        let (t15786, t15787, t15789, t15790, t15794, t15797, t15800) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2007(t15030, t15785, t1241, t1251, t5088, t3598, t1760, t3599, t11606, t225, t4941, t1751, t3481);
        let (t15803, t15806) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2008(t1760, t3630, t3598, t1238, t1252, t14972, t14980, t15787, t15790, t15794, t15797, t15800, t3487, t3593, t3600, t3631, t498, t5055, t5060, t5089);
    (t15786, t15787, t15789, t15790, t15794, t15797, t15800, t15803, t15806)
}
