//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 689/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk689(t2221: f64, t27007: f64, t23571: f64, t3478: f64, t12968: f64, t1359: f64, t2178: f64) -> (f64, f64, f64, f64) {
    let t27008 = t2221 * t27007;
    let t27011 = t23571 * t3478;
    let t27012 = t12968 * t27011;
    let t27015 = t2178 * t1359;
    (t27008, t27011, t27012, t27015)
}
