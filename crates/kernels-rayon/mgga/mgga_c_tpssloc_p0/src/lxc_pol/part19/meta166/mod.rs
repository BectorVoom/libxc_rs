//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk793;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta166(t2710: f64, t798: f64, t116: f64, t229: f64, t212: f64, t776: f64, t2586: f64, t210: f64, t214: f64, t9516: f64, t597: f64, t60: f64, t59: f64, t2386: f64, t131: f64, t207: f64, t2559: f64, t786: f64, t789: f64, t2563: f64, t2582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9520, t9523, t9524, t9525, t9526, t9529, t9533) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk793(t2710, t798, t116, t229, t212, t776, t2586, t210, t214, t9516, t597, t60);
        let (t9534, t9538, t9540, t9541, t9542, t9544) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk794(t59, t9533, t212, t2386, t116, t131, t207, t2559, t786, t789, t2563, t2582);
    (t9520, t9523, t9524, t9525, t9526, t9529, t9534, t9538, t9540, t9541, t9542, t9544)
}
