//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 138/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk138<F: Float>(t342: F, t386: F, t198: F, t293: F, t328: F, t330: F, t336: F, t265: F) -> (F, F, F) {
    let t389 = F::cast_from(1.0_f64) + F::cast_from(0.65854491829355115987e0_f64) * t342 * t386;
    let t390 = F::ln(t389);
    let t393 = t198 * t336 * t390 - t293 + t328 + t330;
    let t394 = t265 < t393;
    let t395 = piecewise3::<F>(t394, t393, t265);
    (t389, t395, t393)
}
