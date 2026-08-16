//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1071/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1071(t86981: f64, t87016: f64, t87050: f64, t87086: f64, t17409: f64, t4805: f64, t20897: f64, t50260: f64, t12664: f64, t20902: f64, t4724: f64, t61366: f64) -> (f64, f64, f64, f64, f64) {
    let t87088 = t86981 + t87016 + t87050 + t87086;
    let t87091 = t17409 * t4805;
    let t87093 = t50260 * t20897;
    let t87095 = t12664 * t20902;
    let t87097 = t61366 * t4724;
    (t87088, t87091, t87093, t87095, t87097)
}
