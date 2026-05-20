//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2245/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2245<F: Float>(t21192: F, t7624: F, t104636: F, t104677: F, t104756: F, t104768: F, t104834: F, t1797: F, t20825: F, t20903: F, t20982: F, t20986: F, t26880: F, t29010: F, t5279: F, t5287: F, t5299: F, t7618: F) -> F {
    let t112279 = t7624 * t21192;
    let t112299 = -F::cast_from(0.38110238327173099531e-3_f64) * t112279 + F::cast_from(0.85748036236139473944e-3_f64) * t104677 * t1797 + t104756 + F::cast_from(0.85748036236139473944e-3_f64) * t29010 * t5287 - F::cast_from(0.45732285992607719436e-2_f64) * t104834 * t1797 + F::cast_from(0.42874018118069736972e-3_f64) * t7618 * t20903 - F::cast_from(0.47637797908966374413e-3_f64) * t26880 * t20825 - F::cast_from(0.11433071498151929859e-2_f64) * t7624 * t20982 - F::cast_from(0.17149607247227894789e-2_f64) * t7624 * t20986 - F::cast_from(0.30488190661738479624e-2_f64) * t104636 * t5299 - F::cast_from(0.30488190661738479624e-2_f64) * t104636 * t5279 + t104768;
    t112299
}
