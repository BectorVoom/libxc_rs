//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 742/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk742<F: Float>(t13187: F, t17104: F, t17360: F, t17362: F, t17422: F, t1901: F, t20875: F, t20880: F, t20884: F, t20888: F, t20894: F, t20899: F, t20904: F, t20909: F, t20912: F, t20916: F, t446: F) -> F {
    let t20919 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t20875 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17104 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t20880 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t20884 - F::cast_from(2.0_f64) * t446 * t20888 + t17360 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t17362 + F::cast_from(2.0_f64) * t446 * t20894 - F::cast_from(2.0_f64) * t446 * t20899 + F::cast_from(2.0_f64) * t446 * t20904 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t17422 - t446 * t20909 - t446 * t20912 / F::cast_from(3.0_f64) - t446 * t20916 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13187;
    t20919
}
