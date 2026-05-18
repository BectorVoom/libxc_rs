//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 893/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk893<F: Float>(t16732: F, t12119: F, t16349: F, t16676: F, t16679: F, t16682: F, t16688: F, t16697: F, t16702: F, t16704: F, t16706: F, t16708: F, t16713: F, t16717: F, t16720: F, t16724: F, t16728: F, t16731: F, t3961: F, t507: F) -> F {
    let t16733 = F::new(0.14739506172839506172e-2) * t16732;
    let t16734 = -F::new(0.33163888888888888888e-2) * t16676 - F::new(0.88437037037037037034e-2) * t16679 + F::new(0.178089025e-1) * t3961 * t16682 + F::new(0.27636574074074074073e-2) * t16688 + F::new(0.73697530864197530861e-2) * t16697 - F::new(0.22109259259259259258e-2) * t12119 - F::new(0.33163888888888888888e-2) * t16702 - F::new(0.33163888888888888888e-2) * t16704 + F::new(0.22109259259259259258e-2) * t16706 - F::new(0.66327777777777777776e-2) * t16708 + F::new(0.55273148148148148146e-2) * t16713 + t16349 * t507 - F::new(0.66327777777777777776e-2) * t16717 - t16720 + F::new(0.33163888888888888888e-2) * t16724 - F::new(0.73697530864197530862e-2) * t16728 + t16731 + t16733;
    t16734
}
