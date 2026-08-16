//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1025/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1025(t3052: f64, t32068: f64, t32069: f64, t3628: f64, t5664: f64, t136159: f64, t136188: f64, t25888: f64, t136189: f64, t137245: f64, t26016: f64, t34482: f64, t358: f64) -> (f64, f64, f64, f64) {
    let t144904 = t5664 * t3628 * t32068 * t32069 * t3052;
    let t144908 = t136159 * t136188 * t32069 * t25888;
    let t144912 = t136159 * t137245 * t136189 * t26016;
    let t144914 = t34482 * t358;
    (t144904, t144908, t144912, t144914)
}
