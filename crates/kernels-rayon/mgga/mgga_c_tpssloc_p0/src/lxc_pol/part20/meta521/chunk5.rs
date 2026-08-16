//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2055/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2055(t2223: f64, t3826: f64, t11985: f64, t25: f64, t514: f64, t11998: f64, t28: f64, t517: f64, t12442: f64, t225: f64, t12036: f64, t12016: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39857 = t2223 * t3826;
    let t39861 = 1.0_f64 / t514 / t11985 / t25;
    let t39877 = 1.0_f64 / t517 / t11998 / t28;
    let t39910 = t12442 * t225;
    let t39913 = t12036 * t225;
    let t39916 = t12016 * t225;
    (t39857, t39861, t39877, t39910, t39913, t39916)
}
