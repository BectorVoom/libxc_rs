//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1427/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1427<F: Float>(t38907: F, t7290: F, t701: F, t321: F, t3720: F, t107: F, t787: F, t2034: F, t28423: F, t28425: F, t28427: F, t28437: F, t28441: F, t28443: F, t28449: F, t28453: F, t33179: F, t33183: F, t33187: F, t33194: F, t33196: F, t4820: F, t6066: F, t7513: F, t7630: F) -> (F, F, F, F) {
    let t39040 = t7290 * t38907;
    let t39044 = t38907 * t701;
    let t39048 = t321 * t3720;
    let t39050 = t787 * t39048 * t107;
    let t39055 = -t28423 + t28425 + t28427 + t28437 - t28441 - t28443 - F::new(0.15889106645266856297e0) * t7513 * t4820 * t39040 + t33179 + t33183 - F::new(0.14300195980740170668e1) * t7630 * t6066 * t39044 + F::new(0.23833659967900284446e0) * t39050 * t2034 + t33187 + F::new(0.72851559312449424385e1) * t28449 + t33194 + F::new(0.76685851907841499354e0) * t28453 + t33196;
    (t39040, t39044, t39048, t39055)
}
