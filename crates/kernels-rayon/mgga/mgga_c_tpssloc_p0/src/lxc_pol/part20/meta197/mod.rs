//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta197 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1189;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1190;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1191;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1192;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1193;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta197(t1118: f64, t4781: f64, t1099: f64, t1670: f64, t3315: f64, t1117: f64, t3313: f64, t3238: f64, t3319: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t1128: f64, t1675: f64, t1136: f64, t1683: f64, t3295: f64, t3339: f64, t3346: f64, t4749: f64, t4757: f64, t4765: f64, t4767: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64, t1137: f64, t1682: f64, t3359: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4782 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1189(t1118, t4781);
        let (t4784, t4785) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1190(t1099, t4782, t1670, t3315);
        let (t4786, t4788, t4794, t4797, t4802) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1191(t1117, t4785, t3313, t3238, t3319, t4721, t4726, t4731, t4735, t1128, t1675, t1136, t1683);
        let t4819 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1192(t3238, t3295, t3339, t3346, t4721, t4726, t4731, t4735, t4749, t4757, t4765, t4767, t4770, t4773, t4776, t4779);
        let t4820 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1193(t1137, t4819);
        let t4823 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1194(t1682, t3359);
    (t4782, t4784, t4785, t4786, t4788, t4794, t4797, t4802, t4819, t4820, t4823)
}
