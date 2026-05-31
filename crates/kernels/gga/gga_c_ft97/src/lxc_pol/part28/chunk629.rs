//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 629/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk629<F: Float>(t26061: F, t492: F, t25873: F, t25876: F, t25881: F, t25886: F, t25891: F, t25897: F, t25902: F, t25906: F, t25910: F, t25913: F, t25917: F, t25921: F) -> (F, F) {
    let t26062 = t26061 * t492;
    let t26077 = -F::cast_from(3.0_f64) * t25873 + t25876 / F::cast_from(6.0_f64) + t25881 / F::cast_from(3.0_f64) - t25886 / F::cast_from(2.0_f64) - t25891 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t25897 + t25902 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) * t25906 + F::cast_from(2.0_f64) * t25910 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t25913 + F::cast_from(2.0_f64) * t25917 - t25921 / F::cast_from(3.0_f64);
    (t26062, t26077)
}
