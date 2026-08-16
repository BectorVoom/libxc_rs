//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk711;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta119(t67: f64, t753: f64, t758: f64, t185: f64, t2250: f64, t707: f64, t152: f64, t32: f64, t2244: f64, t181: f64, t204: f64, t686: f64, t756: f64, t2373: f64, t2377: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2429: f64, t2432: f64, t2450: f64, t2486: f64, t2518: f64, t2520: f64, t2530: f64, t2533: f64, t2537: f64, t2539: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2652, t2653, t2654, t2655, t2657, t2658, t2659, t2661, t2663) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk711(t67, t753, t758, t185, t2250, t707, t152, t32, t2244, t181, t204, t686);
        let (t2665, t2666) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk712(t2663, t756, t2373, t2377, t2408, t2417, t2423, t2426, t2429, t2432, t2450, t2486, t2518, t2520, t2530, t2533, t2537, t2539, t2654, t2657, t2661);
    (t2652, t2653, t2654, t2655, t2657, t2658, t2659, t2661, t2663, t2665, t2666)
}
