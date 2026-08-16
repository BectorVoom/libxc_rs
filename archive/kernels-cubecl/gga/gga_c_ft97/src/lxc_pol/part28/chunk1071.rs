//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1071/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1071<F: Float>(t136241: F, t136243: F, t136250: F, t137070: F, t137073: F, t137090: F, t137102: F, t144946: F, t144950: F, t144953: F, t144956: F, t144961: F, t144966: F, t144970: F, t144974: F, t144978: F) -> F {
    let t145840 = -F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t136241 - t144946 / F::cast_from(36.0_f64) + t136243 / F::cast_from(27.0_f64) - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t144950 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t144953 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t144956 + t144961 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t136250 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t137070 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t137073 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t144966 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t144970 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t144974 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t144978 - t137090 - t137102 / F::cast_from(36.0_f64);
    t145840
}
