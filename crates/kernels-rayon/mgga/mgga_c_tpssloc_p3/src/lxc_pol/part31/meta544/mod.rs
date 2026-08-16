//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1767;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta544(t131: f64, t2587: f64, t81142: f64, t1905: f64, t9537: f64, t81151: f64, t23172: f64, t133: f64, t1891: f64, t6601: f64, t80953: f64, t22816: f64, t23104: f64, t80967: f64, t6612: f64, t812: f64, t836: f64, t2690: f64, t6619: f64, t849: f64, t23132: f64, t2617: f64, t23121: f64, t236: f64, t81613: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81686, t81688, t81715, t81716, t81735, t81742) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1767(t131, t2587, t81142, t1905, t9537, t81151, t23172, t133, t1891, t6601, t80953, t22816, t23104, t80967);
        let (t81749, t81763, t81764, t81769, t81782, t81783) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1768(t6612, t812, t836, t2690, t6619, t849, t23132, t2617, t131, t23121, t9537, t236, t81613);
    (t81686, t81688, t81715, t81716, t81735, t81742, t81749, t81763, t81764, t81769, t81782, t81783)
}
