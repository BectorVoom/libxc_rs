//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2199;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2200;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta690(t19534: f64, t89: f64, t1874: f64, t28030: f64, t6525: f64, t28821: f64, t6880: f64, t28239: f64, t6876: f64, t1983: f64, t26503: f64, t5161: f64, t25010: f64, t7685: f64, t16944: f64, t25014: f64, t25365: f64, t86721: f64, t22960: f64, t67128: f64, t1877: f64, t2219: f64, t7541: f64, t5527: f64, t606: f64, t1915: f64, t22959: f64, t25013: f64, t25024: f64, t2522: f64, t25354: f64, t25358: f64, t25377: f64, t25392: f64, t28241: f64, t28242: f64, t28252: f64, t28256: f64, t28456: f64, t4314: f64, t46341: f64, t6666: f64, t7475: f64, t81539: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97935, t97937, t97941, t97942, t97947) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2199(t19534, t89, t1874, t28030, t6525, t28821, t6880, t28239, t6876, t1983, t26503, t5161);
        let (t97949, t97950, t97953, t97956, t97972, t97985) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2200(t25010, t7685, t16944, t25014, t25365, t86721, t22960, t67128, t1877, t2219, t7541, t5527, t606);
        let t97989 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2201(t1877, t1915, t22959, t25013, t25024, t2522, t25354, t25358, t25377, t25392, t28241, t28242, t28252, t28256, t28456, t4314, t46341, t6666, t7475, t7541, t81539, t97950, t97953, t97956, t97972, t97985);
    (t97935, t97937, t97941, t97942, t97947, t97949, t97972, t97989)
}
