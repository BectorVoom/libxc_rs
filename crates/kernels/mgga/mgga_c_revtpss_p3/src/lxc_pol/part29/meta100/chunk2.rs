//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 605/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk605<F: Float>(t5: F, t2240: F, t2242: F, t2247: F, t2248: F, t2315: F, t603: F, t644: F, t91: F, t117: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t2319 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t2240 * t91 - F::cast_from(8.0_f64) * t2242 * t644 + F::cast_from(20.0_f64) * t2247 * t2248 - F::cast_from(4.0_f64) * t2315 * t603);
    let t2320 = t2319 * t117;
    (t2319, t2320)
}
