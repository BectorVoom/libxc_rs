//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1047/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1047(t1484: f64, t212: f64, t9523: f64, t2586: f64, t213: f64, t4119: f64, t221: f64, t776: f64, t2553: f64, t4128: f64, t2570: f64, t67: f64) -> (f64, f64, f64, f64, f64) {
    let t12984 = t212 * t1484;
    let t12985 = t9523 * t12984;
    let t12986 = t2586 * t12985;
    let t12988 = t213 * t4119;
    let t12990 = t221 * t12988 * t776;
    let t12994 = t221 * t4128 * t2553;
    let t12997 = t2570 * t67;
    (t12984, t12986, t12990, t12994, t12997)
}
