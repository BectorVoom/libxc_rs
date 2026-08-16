//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1573;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1574;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1575;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta283(t10236: f64, t2244: f64, t2987: f64, t3008: f64, t2250: f64, t2989: f64, t2775: f64, t343: f64, t3014: f64, t2262: f64, t972: f64, t2960: f64, t2971: f64, t2970: f64, t2995: f64, t973: f64, t2769: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10237, t10241, t10245, t10254) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1573(t10236, t2244, t2987, t3008, t2250, t2989, t2775, t343);
        let (t10255, t10259, t10263) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1574(t10254, t2244, t2987, t3014, t2262, t972);
        let (t10267, t10274, t10276, t10277) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1575(t2960, t2971, t2970, t2995, t973, t2769, t40);
    (t10237, t10241, t10245, t10254, t10255, t10259, t10263, t10267, t10274, t10276, t10277)
}
