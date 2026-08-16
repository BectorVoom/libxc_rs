//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 992/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk992<F: Float>(t13087: F, t13182: F, t13234: F, t16848: F, t16877: F, t16879: F, t20882: F, t20887: F, t20891: F, t20896: F, t20958: F, t20998: F, t21011: F, t2643: F, t843: F) -> F {
    let t21013 = -F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t13087 - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t13182 + t2643 * t20882 / F::cast_from(256.0_f64) + t2643 * t20887 / F::cast_from(256.0_f64) - t2643 * t20891 / F::cast_from(1024.0_f64) - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t16848 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t843 * t20896 + F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t13234 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t16877 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t16879 + t20958 + t20998 + t21011;
    t21013
}
