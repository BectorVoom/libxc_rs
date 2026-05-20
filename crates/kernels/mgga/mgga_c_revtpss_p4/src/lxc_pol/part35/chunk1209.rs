//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1209/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1209<F: Float>(t101252: F, t101907: F, t109911: F, t109976: F, t109980: F, t110016: F, t110018: F, t110020: F, t110022: F, t114305: F, t114311: F, t114313: F, t2048: F, t29554: F, t7343: F, t7706: F, t7964: F) -> F {
    let t115324 = F::new(40.0) / F::new(3.0) * t110016 + F::new(16.0) / F::new(3.0) * t110018 + F::new(32.0) / F::new(3.0) * t110020 - F::new(60.0) * t101252 * t109911 - F::new(80.0) * t110022 + F::new(88.0) / F::new(9.0) * t101907 - F::new(5.0) / F::new(3.0) * t7343 * t114305 + F::new(10.0) * t109976 * t7706 - F::new(2.0) * t109980 * t114311 - F::new(2.0) / F::new(3.0) * t114313 * t2048 - F::new(2.0) * t29554 * t7964;
    t115324
}
