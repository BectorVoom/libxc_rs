//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta387 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1579;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1580;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1581;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1582;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1583;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1584;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1585;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1586;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1587;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1588;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1589;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta387(t14731: f64, t3240: f64, t123: f64, t3242: f64, t3966: f64, t607: f64, t2250: f64, t4723: f64, t2244: f64, t1088: f64, t3247: f64, t4728: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11247: f64, t14702: f64, t14708: f64, t14721: f64, t14723: f64, t14724: f64, t14728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14733 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1579(t14731, t3240, t123);
        let t14736 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1580(t3242, t3966, t607);
        let t14738 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1581(t14736, t3240, t123);
        let t14740 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1582(t2250, t4723);
        let t14742 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1583(t14740, t3240, t123);
        let t14744 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1584(t2244, t4723);
        let t14746 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1585(t1088, t14744, t123);
        let t14749 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1586(t3247, t3966, t607);
        let t14751 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1587(t1088, t14749, t123);
        let t14753 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1588(t2250, t4728);
        let t14755 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1589(t1088, t14753, t123);
        let t14758 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1590(t11137, t11139, t11141, t11143, t11247, t14702, t14708, t14721, t14723, t14724, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
    (t14733, t14736, t14738, t14740, t14742, t14744, t14746, t14749, t14751, t14753, t14755, t14758)
}
