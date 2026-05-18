//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 719/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk719<F: Float>(t2373: F, t684: F, t9770: F, t446: F, t2409: F, t713: F, t2354: F, t9735: F, t9739: F, t9742: F, t9747: F, t9752: F, t9755: F, t9759: F, t9763: F, t9765: F, t9768: F) -> (F, F, F, F, F, F, F) {
    let t9771 = t684 * t2373;
    let t9772 = t9770 * t9771;
    let t9773 = t446 * t9772;
    let t9775 = t2409 * t713;
    let t9776 = t2354 * t9775;
    let t9777 = t446 * t9776;
    let t9779 = -F::new(2.0) / F::new(27.0) * t9735 - t9739 / F::new(3.0) + t9742 / F::new(3.0) + t9747 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t9752 - t9755 / F::new(9.0) + t9759 / F::new(6.0) + t9763 / F::new(6.0) - t9765 / F::new(9.0) - t9768 / F::new(9.0) - t9773 / F::new(3.0) - t9777 / F::new(3.0);
    (t9771, t9772, t9773, t9775, t9776, t9777, t9779)
}
