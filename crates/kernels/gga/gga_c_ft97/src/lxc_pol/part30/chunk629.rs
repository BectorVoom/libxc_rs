//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 629/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk629<F: Float>(t28140: F, t28141: F, t24737: F, t3842: F, t13885: F, t1901: F, t24567: F, t28102: F, t28106: F, t28110: F, t28113: F, t28116: F, t28120: F, t28125: F, t28130: F, t28133: F, t28137: F, t446: F) -> (F, F) {
    let t28142 = t28140 * t28141;
    let t28145 = t24737 * t3842;
    let t28146 = t13885 * t28145;
    let t28149 = -t446 * t28102 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t24567 + t28106 / F::cast_from(9.0_f64) + t446 * t28110 / F::cast_from(3.0_f64) - t28113 / F::cast_from(9.0_f64) - t446 * t28116 / F::cast_from(3.0_f64) - t446 * t28120 / F::cast_from(3.0_f64) + t1901 * t28125 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t28130 + t1901 * t28133 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t28137 - F::cast_from(2.0_f64) * t1901 * t28142 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t28146;
    (t28145, t28149)
}
