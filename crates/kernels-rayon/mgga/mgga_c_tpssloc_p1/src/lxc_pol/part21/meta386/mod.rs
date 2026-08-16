//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1849;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1850;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1851;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta386(t3109: f64, t4630: f64, t4650: f64, t884: f64, t3071: f64, t10436: f64, t10441: f64, t10449: f64, t10455: f64, t10460: f64, t10490: f64, t10496: f64, t10504: f64, t10511: f64, t10517: f64, t10863: f64, t10866: f64, t10871: f64, t1618: f64, t1622: f64, t3048: f64, t3070: f64, t4636: f64, t3108: f64, t4640: f64, t1611: f64, t3047: f64, t3103: f64, t4641: f64, t1040: f64, t4616: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14059, t14068, t14069, t14074) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1849(t3109, t4630, t4650, t884, t3071, t10436, t10441, t10449, t10455, t10460, t10490, t10496, t10504, t10511, t10517, t10863, t10866, t10871, t1618, t1622, t3048, t3070, t4636);
        let t14077 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1850(t3108, t4640);
        let t14080 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1851(t1611, t3047);
        let (t14084, t14085) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1852(t3103, t4641, t1040, t4616);
    (t14059, t14068, t14069, t14074, t14077, t14080, t14084, t14085)
}
