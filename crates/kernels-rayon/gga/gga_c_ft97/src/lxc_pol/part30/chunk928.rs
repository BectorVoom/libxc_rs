//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 928/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk928(t31538: f64, t6243: f64, t14: f64, t2724: f64, t6250: f64, t19116: f64, t287: f64, t7005: f64, t172: f64, t231: f64, t816: f64, t37481: f64, t40: f64) -> (f64, f64, f64, f64, f64) {
    let t127651 = t31538 * t6243;
    let t127654 = t2724 * t14;
    let t127655 = t127654 * t6250;
    let t127659 = t19116 * t287 * t7005;
    let t127680 = t816 * t172 * t231;
    let t136595 = t40 * t37481;
    (t127651, t127655, t127659, t127680, t136595)
}
