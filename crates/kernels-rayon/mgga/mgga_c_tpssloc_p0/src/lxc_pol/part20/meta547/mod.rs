//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2089;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta547(t212: f64, t2553: f64, t2586: f64, t9523: f64, t9525: f64, t9577: f64, t116: f64, t244: f64, t2379: f64, t2563: f64, t9529: f64, t207: f64, t40419: f64, t9538: f64, t41083: f64, t789: f64, t41011: f64, t9561: f64, t154: f64, t1891: f64, t205: f64, t792: f64, t9558: f64, t118: f64, t794: f64, t9458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41142, t41144, t41149, t41151, t41155) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2089(t212, t2553, t2586, t9523, t9525, t9577, t116, t244, t2379, t2563, t9529, t207, t40419, t9538);
        let (t41156, t41158, t41160, t41161, t41173) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2090(t41083, t789, t41011, t9561, t154, t1891, t205, t792, t9558, t118, t794, t9458);
    (t41142, t41144, t41149, t41151, t41155, t41156, t41158, t41160, t41161, t41173)
}
