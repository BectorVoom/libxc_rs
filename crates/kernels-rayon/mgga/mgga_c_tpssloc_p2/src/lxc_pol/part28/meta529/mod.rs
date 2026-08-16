//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1782;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta529(t22695: f64, t22704: f64, t22705: f64, t22863: f64, t6979: f64, t22641: f64, t3749: f64, t6978: f64, t80854: f64, t22719: f64, t6897: f64, t794: f64, t1984: f64, t80845: f64, t2010: f64, t6973: f64, t80742: f64, t22724: f64, t22727: f64, t22894: f64, t80670: f64, t22882: f64, t22892: f64, t22893: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81050, t81061, t81064, t81066, t81069) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1782(t22695, t22704, t22705, t22863, t6979, t22641, t3749, t6978, t80854, t22719, t6897, t794);
        let (t81071, t81072, t81074, t81076, t81080, t81083) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1783(t1984, t80845, t2010, t6973, t80742, t22724, t22727, t22894, t80670, t22882, t22892, t22893);
    (t81050, t81061, t81064, t81066, t81069, t81071, t81072, t81074, t81076, t81080, t81083)
}
