//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1233;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1234;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1235;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1236;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta199(t1155: f64, t1695: f64, t3238: f64, t3295: f64, t3383: f64, t3390: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t4749: f64, t4757: f64, t4765: f64, t4767: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64, t1156: f64, t1694: f64, t3403: f64, t1129: f64, t1138: f64, t1148: f64, t1157: f64, t1683: f64, t3327: f64, t3332: f64, t3357: f64, t3371: f64, t3376: f64, t3401: f64, t436: f64, t4739: f64, t4742: f64, t4744: f64, t4747: f64, t4784: f64, t4788: f64, t4794: f64, t4797: f64, t4802: f64, t4820: f64, t4824: f64, t4833: f64, t4835: f64, t300: f64, t1687: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4840, t4857) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1233(t1155, t1695, t3238, t3295, t3383, t3390, t4721, t4726, t4731, t4735, t4749, t4757, t4765, t4767, t4770, t4773, t4776, t4779);
        let t4858 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1234(t1156, t4857);
        let t4861 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1235(t1694, t3403);
        let (t4862, t4865) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1236(t1155, t4861, t1129, t1138, t1148, t1157, t1683, t1695, t3327, t3332, t3357, t3371, t3376, t3401, t436, t4739, t4742, t4744, t4747, t4784, t4788, t4794, t4797, t4802, t4820, t4824, t4833, t4835, t4840, t4858);
        let (t4866, t4868, t4869) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1237(t300, t4865, t4833, t1687);
    (t4840, t4857, t4858, t4861, t4862, t4866, t4868, t4869)
}
