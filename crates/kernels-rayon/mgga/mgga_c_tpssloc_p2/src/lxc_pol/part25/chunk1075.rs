//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1075/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1075(t1307: f64, t3914: f64, t12442: f64, t225: f64, t12036: f64, t12016: f64, t12440: f64, t3850: f64, t12167: f64, t562: f64, t12019: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39367 = t1307 * t3914;
    let t39910 = t12442 * t225;
    let t39913 = t12036 * t225;
    let t39916 = t12016 * t225;
    let t39919 = t12440 * t225;
    let t40197 = t1307 * t3850;
    let t40475 = t562 * t12167;
    let t40590 = 1.0_f64 / t12019 / t566;
    (t39367, t39910, t39913, t39916, t39919, t40197, t40475, t40590)
}
