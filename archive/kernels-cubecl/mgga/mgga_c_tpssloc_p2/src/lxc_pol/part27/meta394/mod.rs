//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1613;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1614;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1615;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta394<F: Float>(t3375: F, t4857: F, t1157: F, t1164: F, t3400: F, t4883: F, t3411: F, t4884: F, t225: F, t4947: F, t4943: F, t1734: F, t3590: F, t1246: F, t1235: F, t5011: F, t5072: F, t5079: F, t5068: F, t5075: F, t11883: F, t3507: F, t1755: F, t11871: F, t1751: F, t3493: F, t3625: F, t1932: F, t475: F, t1720: F, t3030: F, t3609: F, t11877: F, t11881: F, t1244: F, t1249: F, t1729: F, t1756: F, t3604: F, t3610: F, t3613: F, t3617: F, t3624: F, t3628: F, t4964: F, t5064: F, t5073: F, t1009: F, t4940: F, t1243: F, t14701: F, t14833: F, t14835: F, t14837: F, t14840: F, t14844: F, t14847: F, t14849: F, t14852: F, t14857: F, t14860: F, t14862: F, t14864: F, t14866: F, t14916: F, t14936: F, t14939: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14963, t14969, t14971, t14972, t14980, t14985) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1613::<F>(t3375, t4857, t1157, t1164, t3400, t4883, t3411, t4884, t225, t4947, t4943, t1734, t3590);
        let (t14986, t14989, t14992, t14997, t15001, t15004, t15009) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1614::<F>(t1246, t14985, t1235, t5011, t5072, t5079, t5068, t5075, t11883, t3507, t1755, t11871);
        let (t15018, t15026, t15030) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1615::<F>(t1751, t3493, t1246, t3507, t3625, t1932, t475, t1755, t1720, t3030, t3609, t11877, t11881, t1244, t1249, t14986, t14989, t14992, t14997, t15001, t15004, t15009, t1729, t1756, t3604, t3610, t3613, t3617, t3624, t3628, t4964, t5064, t5073);
        let (t15031, t15032, t15035) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1616::<F>(t1009, t4940, t1243, t14701, t14833, t14835, t14837, t14840, t14844, t14847, t14849, t14852, t14857, t14860, t14862, t14864, t14866, t14916, t14936, t14939);
    (t14963, t14969, t14971, t14972, t14980, t15018, t15026, t15030, t15031, t15032, t15035)
}
