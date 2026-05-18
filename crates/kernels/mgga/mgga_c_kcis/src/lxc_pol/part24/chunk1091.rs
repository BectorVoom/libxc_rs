//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1091/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1091<F: Float>(t26685: F, t27799: F, t27804: F, t27849: F, t27857: F, t28905: F, t28909: F, t28913: F, t28917: F, t28920: F, t28925: F, t28928: F) -> F {
    let t28931 = F::new(0.22109259259259259258e-2) * t27799 - F::new(0.15445601851851851852e-3) * t27804 + F::new(0.33163888888888888888e-2) * t28905 + F::new(0.16581944444444444444e-2) * t28909 + F::new(0.27636574074074074073e-2) * t28913 - F::new(0.33163888888888888888e-2) * t28917 + F::new(0.24872916666666666666e-2) * t28920 + F::new(0.22109259259259259258e-2) * t27849 + F::new(0.46336805555555555556e-3) * t27857 - F::new(0.33163888888888888888e-2) * t28925 + F::new(0.61836467013888888889e-4) * t26685 * t28928;
    t28931
}
