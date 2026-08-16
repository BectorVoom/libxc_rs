//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 872/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk872(t11763: f64, t2028: f64, t2536: f64, t787: f64, t13506: f64, t4673: f64, t6060: f64, t2087: f64, t4614: f64, t43715: f64, t10931: f64, t23220: f64, t45316: f64) -> (f64, f64, f64, f64, f64) {
    let t45557 = 0.39722766613167140743e-1_f64 * t787 * t2536 * t11763 * t2028;
    let t45560 = 0.14300195980740170667e1_f64 * t6060 * t4673 * t13506;
    let t45563 = 0.82820720060468819301e2_f64 * t2087 * t4614 * t13506;
    let t45565 = 0.23833659967900284446e0_f64 * t43715;
    let t45569 = 0.27606906686822939767e2_f64 * t23220 * t10931 * t45316;
    (t45557, t45560, t45563, t45565, t45569)
}
