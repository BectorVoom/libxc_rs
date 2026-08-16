//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 510/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk510<F: Float>(t3938: F, t734: F, t91: F, t3688: F, t3710: F, t2339: F, t2342: F, t2533: F, t3693: F, t3697: F, t3702: F, t3707: F, t3715: F, t3720: F, t3824: F, t3904: F) -> (F, F, F, F) {
    let t3940 = t91 * t734 * t3938;
    let t3942 = t3688 / F::cast_from(27.0_f64);
    let t3947 = t3710 / F::cast_from(9.0_f64);
    let t3951 = -t3904 / F::cast_from(12.0_f64) + t3940 / F::cast_from(6.0_f64) + t2533 + t2339 + t2342 + t3942 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t3693 + t3697 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3702 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3707 + t3947 + t3715 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3720 - t3824 / F::cast_from(3.0_f64);
    (t3940, t3942, t3947, t3951)
}
