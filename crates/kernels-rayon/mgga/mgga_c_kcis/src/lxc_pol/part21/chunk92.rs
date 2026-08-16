//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 92/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk92(t250: f64, t252: f64, t253: f64, t242: f64, t245: f64, t248: f64) -> (f64, f64, f64, f64) {
    let t255 = t250 * t252 * t253;
    let t257 = 0.379785e1_f64 * t245 + 0.8969e0_f64 * t242 + 0.204775e0_f64 * t248 + 0.123235e0_f64 * t255;
    let t260 = 1.0_f64 + 0.16081824322151104822e2_f64 / t257;
    let t261 = f64::ln(t260);
    (t255, t257, t260, t261)
}
