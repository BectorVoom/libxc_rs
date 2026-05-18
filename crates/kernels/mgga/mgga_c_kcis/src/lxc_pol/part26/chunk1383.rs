//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1383/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1383<F: Float>(t28426: F, t8144: F, t102725: F, t102729: F, t102731: F, t102740: F, t102743: F, t102746: F, t28480: F, t29404: F, t7901: F, t8159: F, t98909: F, t98911: F, t98918: F) -> F {
    let t103749 = t8144 * t28426;
    let t103762 = F::new(0.46336805555555555557e-3) * t103749 + t98909 + t98911 + F::new(0.33163888888888888888e-2) * t102725 - F::new(0.33163888888888888888e-2) * t102729 - F::new(0.36848765432098765431e-3) * t102731 + F::new(0.67960648148148148147e-2) * t29404 * t7901 - F::new(0.37069444444444444444e-2) * t28480 * t8159 - F::new(0.22109259259259259259e-2) * t98918 + F::new(0.13265555555555555555e-1) * t102740 - F::new(0.13265555555555555555e-1) * t102743 + F::new(0.24320185185185185185e-1) * t102746;
    t103762
}
