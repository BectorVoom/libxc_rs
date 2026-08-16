//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1932;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1933;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta416(t1166: f64, t14858: f64, t3419: f64, t4869: f64, t11180: f64, t1671: f64, t3259: f64, t4782: f64, t14704: f64, t14710: f64, t14722: f64, t11215: f64, t11217: f64, t14720: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t14766: f64, t14781: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14728: f64, t14809: f64, t14811: f64, t14814: f64, t14816: f64, t14818: f64, t14824: f64, t11195: f64, t11204: f64, t11211: f64, t11213: f64, t14702: f64, t14708: f64, t14713: f64, t14759: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t14802: f64, t14805: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14860, t14862, t14864, t14866, t14868, t14870, t14886, t14887) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1932(t1166, t14858, t3419, t4869, t11180, t1671, t3259, t4782, t14704, t14710, t14722, t11215, t11217, t14720, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
        let (t14890, t14911) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1933(t14781, t11137, t11139, t11141, t11143, t14728, t14809, t14811, t14814, t14816, t14818, t14824);
        let t14913 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1934(t11195, t11204, t11211, t11213, t14702, t14708, t14713, t14759, t14779, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t14868, t14870, t14887, t14890, t14911);
    (t14860, t14862, t14864, t14866, t14868, t14870, t14886, t14890, t14913)
}
