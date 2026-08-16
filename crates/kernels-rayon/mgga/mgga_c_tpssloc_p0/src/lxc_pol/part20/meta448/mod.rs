//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1897;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta448(t1751: f64, t3493: f64, t1246: f64, t3507: f64, t3625: f64, t1932: f64, t475: f64, t1755: f64, t1720: f64, t3030: f64, t3609: f64, t11877: f64, t11881: f64, t1244: f64, t1249: f64, t14986: f64, t14989: f64, t14992: f64, t14997: f64, t15001: f64, t15004: f64, t15009: f64, t1729: f64, t1756: f64, t3604: f64, t3610: f64, t3613: f64, t3617: f64, t3624: f64, t3628: f64, t4964: f64, t5064: f64, t5073: f64, t1009: f64, t4940: f64, t1243: f64, t14701: f64, t14833: f64, t14835: f64, t14837: f64, t14840: f64, t14844: f64, t14847: f64, t14849: f64, t14852: f64, t14857: f64, t14860: f64, t14862: f64, t14864: f64, t14866: f64, t14916: f64, t14936: f64, t14939: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15016, t15018, t15019, t15022, t15023, t15026, t15027, t15030) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1897(t1751, t3493, t1246, t3507, t3625, t1932, t475, t1755, t1720, t3030, t3609, t11877, t11881, t1244, t1249, t14986, t14989, t14992, t14997, t15001, t15004, t15009, t1729, t1756, t3604, t3610, t3613, t3617, t3624, t3628, t4964, t5064, t5073);
        let (t15031, t15032, t15035) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1898(t1009, t4940, t1243, t14701, t14833, t14835, t14837, t14840, t14844, t14847, t14849, t14852, t14857, t14860, t14862, t14864, t14866, t14916, t14936, t14939);
    (t15016, t15018, t15019, t15022, t15023, t15026, t15027, t15030, t15031, t15032, t15035)
}
