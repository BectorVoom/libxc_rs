//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 866/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk866<F: Float>(t16710: F, t16714: F, t16717: F, t16721: F, t16724: F, t16727: F, t16730: F, t16734: F, t17237: F, t17241: F, t17244: F, t16739: F, t16742: F, t16745: F, t16748: F, t16751: F, t16756: F, t16760: F, t16922: F, t16925: F, t16928: F, t17349: F) -> (F, F) {
    let t17472 = F::new(4.0) / F::new(9.0) * t16710 - F::new(2.0) / F::new(9.0) * t16714 - F::new(2.0) / F::new(3.0) * t16717 + F::new(2.0) / F::new(27.0) * t16721 + F::new(4.0) / F::new(9.0) * t16724 - F::new(10.0) / F::new(81.0) * t16727 - F::new(8.0) / F::new(27.0) * t16730 + F::new(2.0) / F::new(9.0) * t16734 - t17237 / F::new(12.0) + t17241 / F::new(8.0) - t17244 / F::new(6.0);
    let t17484 = -F::new(2.0) * t16739 + F::new(4.0) / F::new(3.0) * t16742 + t16745 / F::new(27.0) - F::new(2.0) / F::new(27.0) * t16748 + F::new(2.0) / F::new(81.0) * t16751 + F::new(2.0) / F::new(3.0) * t16756 - t16760 / F::new(9.0) + t17349 / F::new(6.0) - t16922 / F::new(3.0) + t16925 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t16928;
    (t17472, t17484)
}
