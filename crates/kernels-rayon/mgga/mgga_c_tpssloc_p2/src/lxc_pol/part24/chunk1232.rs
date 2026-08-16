//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1232/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1232(t85: f64, t24: f64, t111: f64, t9346: f64, t1307: f64, t3914: f64, t12442: f64, t225: f64, t12036: f64, t12016: f64, t12440: f64, t3850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t39235 = t9346 * t111;
    let t39367 = t1307 * t3914;
    let t39910 = t12442 * t225;
    let t39913 = t12036 * t225;
    let t39916 = t12016 * t225;
    let t39919 = t12440 * t225;
    let t40197 = t1307 * t3850;
    (t39063, t39235, t39367, t39910, t39913, t39916, t39919, t40197)
}
