//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta376 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1401;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1402;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1403;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1404;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1405;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1406;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1407;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1408;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta376(t43748: f64, t43750: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t11778: f64, t154: f64, t123: f64, t43764: f64, t1091: f64, t9698: f64, t22715: f64, t268: f64, t405: f64, t3240: f64, t43752: f64, t1088: f64, t43757: f64, t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t1107: f64, t11223: f64, t699: f64, t11205: f64, t11208: f64, t11219: f64, t136: f64, t43792: f64, t3297: f64, t43796: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43808, t43809) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1401(t43748, t43750, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t11778, t154);
        let t43811 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1402(t123, t43764, t43809);
        let t43816 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1403(t1091, t9698);
        let t43819 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1404(t22715, t268, t405);
        let (t43820, t43823) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1405(t43819, t123, t3240, t43752);
        let t43828 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1406(t1088, t123, t43757);
        let t43831 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1407(t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43811, t43816, t43820, t43823, t43828);
        let (t43832, t43833, t43835, t43837, t43839, t43842, t43845) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1408(t43808, t43831, t1107, t11223, t699, t11205, t11208, t11219, t136, t43792, t3297, t43796);
    (t43811, t43816, t43819, t43823, t43828, t43832, t43833, t43835, t43837, t43839, t43842, t43845)
}
