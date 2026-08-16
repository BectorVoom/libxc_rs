//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2037/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2037(t87487: f64, t22996: f64, t6590: f64, t23110: f64, t25299: f64, t81651: f64, t23168: f64, t25313: f64, t252: f64, t87230: f64, t25321: f64, t25284: f64, t6579: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87488 = 0.28260929265898273598e-2_f64 * t87487;
    let t87504 = t6590 * t22996;
    let t87520 = t81651 * t23110 * t25299;
    let t87521 = 0.16449340668482264365e-1_f64 * t87520;
    let t87522 = t23168 * t25313;
    let t87523 = 0.76763589786250567036e-1_f64 * t87522;
    let t87529 = t87230 * t252;
    let t87533 = t23168 * t25321;
    let t87534 = 0.76763589786250567036e-1_f64 * t87533;
    let t87535 = t6579 * t25284;
    (t87488, t87504, t87521, t87523, t87529, t87534, t87535)
}
