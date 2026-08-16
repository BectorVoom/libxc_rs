//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2007;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta609(t81735: f64, t1891: f64, t22816: f64, t23104: f64, t80967: f64, t6612: f64, t812: f64, t836: f64, t2690: f64, t6619: f64, t849: f64, t23132: f64, t2617: f64, t131: f64, t23121: f64, t9537: f64, t236: f64, t81613: f64, t22822: f64, t281: f64, t6589: f64, t23124: f64, t23076: f64, t6597: f64, t23047: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81736, t81743, t81749, t81763, t81764, t81769) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2007(t81735, t1891, t22816, t23104, t80967, t6612, t812, t836, t2690, t6619, t849, t23132, t2617);
        let (t81782, t81783, t81788, t81789, t81792, t81803) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2008(t131, t23121, t9537, t236, t81613, t22822, t281, t6589, t23124, t23076, t6597, t23047, t2617);
    (t81736, t81743, t81749, t81763, t81764, t81769, t81782, t81783, t81788, t81789, t81792, t81803)
}
