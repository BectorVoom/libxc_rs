//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1383;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1384;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1385;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1386;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1387;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1388;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1389;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta373(t11147: f64, t2244: f64, t2250: f64, t11145: f64, t123: f64, t3240: f64, t43711: f64, t43715: f64, t1088: f64, t43719: f64, t43723: f64, t2394: f64, t3244: f64, t11149: f64, t690: f64, t3242: f64, t39103: f64, t136: f64, t3297: f64, t43713: f64, t43717: f64, t43721: f64, t43725: f64, t43727: f64, t43729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43732, t43734) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1383(t11147, t2244, t2250, t11145, t123);
        let t43737 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1384(t123, t3240, t43711);
        let t43740 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1385(t123, t3240, t43715);
        let t43743 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1386(t1088, t123, t43719);
        let t43746 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1387(t1088, t123, t43723);
        let t43748 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1388(t2394, t3244);
        let t43750 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1389(t11149, t690);
        let (t43752, t43754, t43756) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1390(t3242, t39103, t136, t3297, t43713, t43717, t43721, t43725, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43748, t43750);
    (t43732, t43734, t43737, t43740, t43743, t43746, t43748, t43750, t43752, t43754, t43756)
}
