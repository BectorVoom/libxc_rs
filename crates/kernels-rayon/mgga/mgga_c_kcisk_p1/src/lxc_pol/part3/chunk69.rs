//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 69/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk69(t139: f64, t175: f64, t197: f64, t198: f64, t201: f64, t190: f64, t116: f64, t167: f64) -> (f64, f64, f64) {
    let t205 = 0.619125e-2_f64 * t197 * t198 - 0.79593333333333333331e-1_f64 * t139 * t201 * t175;
    let t206 = t205 * t190;
    let t207 = t116 * t167;
    (t205, t206, t207)
}
