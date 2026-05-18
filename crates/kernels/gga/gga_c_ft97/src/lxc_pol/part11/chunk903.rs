//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 903/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk903<F: Float>(t37391: F, t464: F, t1775: F, t8263: F, t8278: F, t11755: F, t11756: F, t11761: F, t1588: F, t1755: F, t1800: F, t2: F, t24: F, t37415: F, t37430: F, t38254: F, t38504: F, t38506: F, t38508: F, t38513: F, t38519: F, t38525: F, t38526: F, t432: F, t462: F, t463: F, t469: F, t7750: F, t7815: F, t92: F) -> (F, F) {
    let t38534 = t464 * t37391;
    let t38538 = t1775 * t8263;
    let t38545 = t1775 * t8278;
    let t38547 = F::new(112.0) / F::new(27.0) * t38504 + F::new(8.0) * t38506 + F::new(24.0) * t92 * t24 * t38508 * t37430 - F::new(8.0) / F::new(3.0) * t38513 + F::new(6.0) * t92 * t24 * t1800 * t37415 + F::new(16.0) / F::new(3.0) * t38519 - t92 * t24 * t469 * t38254 + t38525 + F::new(8.0) / F::new(3.0) * t11755 * t11756 * t38526 - F::new(8.0) * t11761 * t1800 * t432 * t7815 - t462 * t463 * t38534 / F::new(3.0) - F::new(8.0) * t38538 - F::new(36.0) * t462 * t7750 * t2 * t1588 * t1755 + F::new(40.0) / F::new(81.0) * t38545;
    (t38534, t38547)
}
