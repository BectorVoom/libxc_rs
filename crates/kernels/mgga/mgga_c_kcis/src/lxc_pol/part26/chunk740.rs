//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 740/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk740<F: Float>(t2260: F, t7968: F, t7976: F, t7978: F, t7991: F, t8166: F, t8169: F, t8172: F, t8177: F, t8180: F, t8209: F, t8213: F, t8218: F, t8222: F, t8226: F) -> F {
    let t8236 = -F::new(0.34752604166666666667e-3) * t8209 * t2260 + F::new(0.46377350260416666667e-4) * t7968 * t8213 + F::new(0.92673611111111111112e-3) * t8218 * t2260 - t7976 - F::new(0.11584201388888888889e-3) * t7978 * t8222 + F::new(0.34752604166666666667e-3) * t7978 * t8226 + F::new(0.34752604166666666667e-3) * t7978 * t8213 + t7991 + F::new(0.11607361111111111111e-2) * t8166 + F::new(0.17411041666666666666e-2) * t8169 - F::new(0.17411041666666666666e-2) * t8172 - F::new(0.46429444444444444443e-2) * t8177 + F::new(0.11607361111111111111e-2) * t8180;
    t8236
}
