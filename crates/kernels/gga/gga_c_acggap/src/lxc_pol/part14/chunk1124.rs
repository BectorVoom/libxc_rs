//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1124/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1124<F: Float>(t31362: F, t9589: F, t4680: F, t7337: F, t9588: F, t1181: F, t26995: F, t599: F, t30786: F, t30790: F, t34866: F, t34894: F, t34896: F, t34946: F, t34958: F, t34962: F, t37287: F, t37293: F, t39551: F, t39555: F, t39557: F, t39559: F, t39563: F) -> F {
    let t39567 = t31362 * t9589;
    let t39570 = t7337 * t4680 * t9588;
    let t39574 = t7337 * t1181 * t599 * t26995;
    let t39576 = -F::new(0.10718504529517434243e-3) * t39551 - F::new(0.10718504529517434243e-3) * t39555 - t34866 + t37287 + t34894 + t34896 - t37293 - t39557 / F::new(24.0) - t39559 / F::new(24.0) - F::new(0.94344276868812456204e-2) * t39563 - t34946 + t34958 - t34962 - F::new(0.10718504529517434243e-3) * t30786 - F::new(0.14291339372689912324e-3) * t30790 + F::new(0.10718504529517434243e-2) * t39567 + F::new(0.10718504529517434243e-2) * t39570 + F::new(0.10718504529517434243e-2) * t39574;
    t39576
}
