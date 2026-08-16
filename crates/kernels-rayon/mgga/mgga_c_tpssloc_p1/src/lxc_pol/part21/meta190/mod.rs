//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta190 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1188;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1189;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1190;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1191;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1192;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta190(t1615: f64, t376: f64, t1022: f64, t3131: f64, t4582: f64, t1023: f64, t135: f64, t1606: f64, t973: f64, t3966: f64, t998: f64, t974: f64, t1041: f64, t1607: f64, t1622: f64, t2960: f64, t3039: f64, t3048: f64, t3054: f64, t3070: f64, t3084: f64, t3092: f64, t3130: f64, t4562: f64, t4565: f64, t4572: f64, t4575: f64, t4579: f64, t4585: f64, t4590: f64, t225: f64, t4552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4593 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1188(t1615, t376);
        let t4594 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1189(t1022, t3131);
        let (t4595, t4596) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1190(t4593, t4594, t4582);
        let (t4599, t4600) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1191(t1023, t4593, t4582);
        let (t4603, t4604, t4608, t4609, t4613) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1192(t135, t1606, t973, t3966, t998, t974, t1041, t1607, t1622, t2960, t3039, t3048, t3054, t3070, t3084, t3092, t3130, t4562, t4565, t4572, t4575, t4579, t4585, t4590, t4596, t4600);
        let t4615 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1193(t225, t4552);
    (t4593, t4594, t4595, t4596, t4599, t4600, t4603, t4604, t4608, t4609, t4613, t4615)
}
