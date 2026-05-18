//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1369/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1369<F: Float>(t1250: F, t251: F, t47323: F, t27030: F, t27070: F, t28153: F, t28190: F, t7775: F, t8087: F, t92604: F, t93056: F, t96141: F, t96154: F, t96157: F, t96160: F, t96178: F, t96181: F, t96184: F) -> F {
    let t97297 = t47323 * t251 * t1250;
    let t97303 = -F::new(0.11607361111111111111e-2) * t96141 + F::new(0.23214722222222222222e-2) * t96154 + F::new(0.61905925925925925926e-2) * t96157 - F::new(0.18534722222222222222e-2) * t92604 * t8087 + F::new(0.11607361111111111111e-2) * t96160 + F::new(0.46377350260416666667e-4) * t93056 * t8087 + F::new(0.92754700520833333334e-4) * t27070 * t28153 - F::new(0.69505208333333333334e-3) * t28190 * t27030 + F::new(0.92754700520833333334e-4) * t97297 * t7775 + F::new(0.11607361111111111111e-2) * t96178 - F::new(0.38691203703703703703e-3) * t96181 - F::new(0.17411041666666666666e-2) * t96184;
    t97303
}
