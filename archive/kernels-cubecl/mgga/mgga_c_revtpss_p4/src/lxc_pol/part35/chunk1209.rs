//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1209/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1209<F: Float>(t101252: F, t101907: F, t109911: F, t109976: F, t109980: F, t110016: F, t110018: F, t110020: F, t110022: F, t114305: F, t114311: F, t114313: F, t2048: F, t29554: F, t7343: F, t7706: F, t7964: F) -> F {
    let t115324 = F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t110016 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t110018 + F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t110020 - F::cast_from(60.0_f64) * t101252 * t109911 - F::cast_from(80.0_f64) * t110022 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t101907 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7343 * t114305 + F::cast_from(10.0_f64) * t109976 * t7706 - F::cast_from(2.0_f64) * t109980 * t114311 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t114313 * t2048 - F::cast_from(2.0_f64) * t29554 * t7964;
    t115324
}
