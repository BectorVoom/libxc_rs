//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2255/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2255<F: Float>(t104647: F, t104721: F, t104853: F, t104888: F, t104994: F, t104999: F, t105002: F, t105007: F, t105014: F, t20767: F, t20880: F, t21037: F, t21173: F, t21223: F, t26880: F, t29097: F, t29100: F, t5402: F) -> F {
    let t112531 = -t104994 - F::cast_from(0.19055119163586549765e-3_f64) * t104999 + t105002 - t105007 - F::cast_from(0.57165357490759649296e-3_f64) * t104888 * t5402 + F::cast_from(0.30488190661738479624e-2_f64) * t104721 * t5402 - F::cast_from(0.57165357490759649296e-3_f64) * t29097 * t21223 + F::cast_from(0.28582678745379824648e-3_f64) * t29100 * t21173 + F::cast_from(0.11433071498151929859e-2_f64) * t104647 * t21037 + t105014 + F::cast_from(0.57165357490759649296e-3_f64) * t26880 * t20880 - F::cast_from(0.11433071498151929859e-2_f64) * t104853 * t20767;
    t112531
}
