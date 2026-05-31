//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 934/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk934<F: Float>(t10007: F, t18446: F, t1882: F, t5066: F, t13959: F, t13961: F, t13963: F, t13965: F, t14018: F, t14020: F, t14052: F, t18431: F, t18434: F, t18439: F, t18443: F, t1901: F, t9822: F, t9824: F) -> F {
    let t18447 = t10007 * t18446;
    let t18452 = t1882 * t5066;
    let t18454 = -t13959 - t13961 - t13963 + t13965 - t14018 - t14020 + t18431 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t18434 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t18439 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t18443 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t18447 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9822 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9824 - t14052 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18452;
    t18454
}
