//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2101;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2102;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2103;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2104;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2105;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2106;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2107;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2108;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2109;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta554(t41654: f64, t2394: f64, t2781: f64, t2772: f64, t10565: f64, t690: f64, t10574: f64, t10969: f64, t154: f64, t2769: f64, t2777: f64, t10568: f64, t10529: f64, t10571: f64, t885: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41655, t41656) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2101(t41654, t2394, t2781);
        let t41658 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2102(t2394, t2772);
        let t41660 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2103(t10565, t690);
        let t41662 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2104(t10574, t690);
        let (t41664, t41666, t41675) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2105(t10969, t154, t2769, t2394, t2777);
        let t41678 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2106(t10568, t690);
        let t41680 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2107(t10529, t690);
        let t41682 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2108(t10571, t690);
        let t41684 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2109(t885, t9698);
    (t41655, t41656, t41658, t41660, t41662, t41664, t41666, t41675, t41678, t41680, t41682, t41684)
}
