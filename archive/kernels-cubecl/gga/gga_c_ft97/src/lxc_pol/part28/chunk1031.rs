//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1031/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1031<F: Float>(t136116: F, t28: F, t89: F, t942: F, t136241: F, t136243: F, t136250: F, t137070: F, t137073: F, t137102: F, t137623: F, t144946: F, t144950: F, t144953: F, t144956: F, t144961: F, t144966: F, t144970: F, t144974: F) -> (F, F) {
    let t144978 = t89 * t28 * t136116 * t942;
    let t144981 = -F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t136241 - t144946 / F::cast_from(12.0_f64) + t136243 / F::cast_from(9.0_f64) - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t144950 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t144953 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t144956 + t144961 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t136250 + F::cast_from(2.0_f64) * t137070 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t137073 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t144966 + F::cast_from(2.0_f64) * t144970 + F::cast_from(2.0_f64) * t144974 + F::cast_from(2.0_f64) * t144978 - t137623 - t137102 / F::cast_from(12.0_f64);
    (t144978, t144981)
}
