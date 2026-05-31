//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 686/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk686<F: Float>(t574: F, t616: F, t6615: F, t1901: F, t26925: F, t26929: F, t26932: F, t26936: F, t26940: F, t26943: F, t26947: F, t26952: F, t26957: F, t26961: F, t26965: F, t26969: F, t446: F) -> F {
    let t26973 = t574 * t616 * t6615;
    let t26976 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t26925 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t26929 + t1901 * t26932 / F::cast_from(9.0_f64) - t1901 * t26936 / F::cast_from(9.0_f64) + t446 * t26940 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t26943 + t446 * t26947 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t26952 + t446 * t26957 / F::cast_from(3.0_f64) - t446 * t26961 / F::cast_from(3.0_f64) - t446 * t26965 / F::cast_from(3.0_f64) - t446 * t26969 / F::cast_from(3.0_f64) - t446 * t26973 / F::cast_from(3.0_f64);
    t26976
}
