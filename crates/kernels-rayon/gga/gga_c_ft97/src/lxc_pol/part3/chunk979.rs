//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 979/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk979(t19172: f64, t231: f64, t3750: f64, t230: f64, t4939: f64, t803: f64, t5252: f64, t688: f64, t1193: f64, t5255: f64, t18127: f64, t278: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19174 = t231 * t19172 * t3750;
    let t19177 = t230 * t4939;
    let t19178 = t19177 * t803;
    let t19181 = t5252 * t688;
    let t19184 = t1193 * t3750;
    let t19189 = t5255 * t688;
    let t19192 = t18127 * t278;
    (t19174, t19178, t19181, t19184, t19189, t19192)
}
