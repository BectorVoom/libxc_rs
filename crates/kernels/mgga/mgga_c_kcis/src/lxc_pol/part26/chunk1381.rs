//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1381/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1381<F: Float>(t102674: F, t102678: F, t102681: F, t102684: F, t102687: F, t102694: F, t102698: F, t102701: F, t2239: F, t23157: F, t28403: F, t29404: F, t3964: F, t7916: F, t8151: F, t98874: F) -> F {
    let t103731 = F::new(0.16581944444444444444e-2) * t102674 - F::new(0.67960648148148148147e-2) * t3964 * t23157 * t2239 - F::new(0.37069444444444444444e-2) * t8151 * t28403 + F::new(0.67960648148148148147e-2) * t29404 * t7916 + F::new(0.13265555555555555555e-1) * t102678 - F::new(0.82376543209876543213e-3) * t98874 - F::new(0.55273148148148148147e-3) * t102681 + F::new(0.11054629629629629629e-2) * t102684 - F::new(0.33163888888888888888e-2) * t102687 + F::new(0.11054629629629629629e-2) * t102694 + F::new(0.22109259259259259258e-2) * t102698 - F::new(0.44218518518518518516e-2) * t102701;
    t103731
}
