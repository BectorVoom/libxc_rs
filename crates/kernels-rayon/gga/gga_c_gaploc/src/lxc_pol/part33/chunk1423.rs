//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1423/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1423(t38907: f64, t7290: f64, t701: f64, t321: f64, t3720: f64, t107: f64, t787: f64, t2034: f64, t28423: f64, t28425: f64, t28427: f64, t28437: f64, t28441: f64, t28443: f64, t28449: f64, t28453: f64, t33179: f64, t33183: f64, t33187: f64, t33194: f64, t33196: f64, t4820: f64, t6066: f64, t7513: f64, t7630: f64) -> (f64, f64, f64, f64) {
    let t39040 = t7290 * t38907;
    let t39044 = t38907 * t701;
    let t39048 = t321 * t3720;
    let t39050 = t787 * t39048 * t107;
    let t39055 = -t28423 + t28425 + t28427 + t28437 - t28441 - t28443 - 0.15889106645266856297e0_f64 * t7513 * t4820 * t39040 + t33179 + t33183 - 0.14300195980740170668e1_f64 * t7630 * t6066 * t39044 + 0.23833659967900284446e0_f64 * t39050 * t2034 + t33187 + 0.72851559312449424385e1_f64 * t28449 + t33194 + 0.76685851907841499354e0_f64 * t28453 + t33196;
    (t39040, t39044, t39048, t39055)
}
