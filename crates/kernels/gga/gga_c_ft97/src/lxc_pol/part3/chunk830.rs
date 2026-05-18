//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 830/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk830<F: Float>(t140: F, t16802: F, t16917: F, t526: F, t27: F, t89: F, t375: F, t4715: F, t4669: F, t12918: F, t16706: F, t16710: F, t16714: F, t16717: F, t16721: F, t16724: F, t16727: F, t16730: F, t16734: F, t16739: F, t16742: F, t16745: F, t16748: F, t16751: F, t16756: F, t16760: F) -> (F, F, F, F, F) {
    let t141 = F::new(0.1e-59) < t140;
    let t16919 = piecewise3::<f64>(t141, t16802 + t16917, F::new(0.0));
    let t16920 = t526 * t16919;
    let t16922 = t89 * t27 * t16920;
    let t16925 = t89 * t375 * t4715;
    let t16928 = t89 * t375 * t4669;
    let t16930 = -t12918 - t16706 / F::new(27.0) + F::new(2.0) / F::new(9.0) * t16710 - t16714 / F::new(9.0) - t16717 / F::new(3.0) + t16721 / F::new(27.0) + F::new(2.0) / F::new(9.0) * t16724 - F::new(5.0) / F::new(81.0) * t16727 - F::new(4.0) / F::new(27.0) * t16730 + t16734 / F::new(9.0) - t16739 + F::new(2.0) / F::new(3.0) * t16742 + t16745 / F::new(54.0) - t16748 / F::new(27.0) + t16751 / F::new(81.0) + t16756 / F::new(3.0) - t16760 / F::new(18.0) - t16922 / F::new(6.0) + t16925 / F::new(18.0) - t16928 / F::new(9.0);
    (t16919, t16922, t16925, t16928, t16930)
}
