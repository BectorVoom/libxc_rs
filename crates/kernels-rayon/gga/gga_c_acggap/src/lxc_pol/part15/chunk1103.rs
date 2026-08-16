//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1103/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1103(t5924: f64, t8511: f64, t6265: f64, t2030: f64, t5906: f64, t7815: f64, t6289: f64, t7440: f64, t9666: f64, t1988: f64, t9554: f64, t1782: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39052 = t8511 * t5924;
    let t39054 = t8511 * t6265;
    let t39057 = t2030 * t7815 * t5906;
    let t39060 = t2030 * t7815 * t6289;
    let t39062 = t7440 * t9666;
    let t39064 = t1988 * t9554;
    let t39066 = t355 * t1782;
    (t39052, t39054, t39057, t39060, t39062, t39064, t39066)
}
