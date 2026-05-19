//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1365/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1365<F: Float>(t3636: F, t5381: F, t1260: F, t12966: F, t16775: F, t247: F, t3719: F, t1222: F, t1261: F, t17232: F, t17237: F, t17243: F, t17244: F, t17247: F, t17250: F, t17254: F, t17258: F, t5384: F, t5386: F) -> F {
    let t17260 = F::cast_from(0.19055119163586549765e-3_f64) * t5381 * t3636;
    let t17261 = t12966 * t1260;
    let t17265 = t247 * t3719 * t16775;
    let t17268 = -F::cast_from(0.57165357490759649296e-3_f64) * t1261 * t17232 - F::cast_from(0.63517063878621832552e-3_f64) * t1261 * t17237 - t17243 - t1222 * t17244 / F::new(72.0) - t1222 * t17247 / F::new(144.0) - t1222 * t17250 / F::new(48.0) + F::cast_from(0.85748036236139473944e-3_f64) * t5384 * t17254 + t17258 - t17260 + F::cast_from(0.85748036236139473944e-3_f64) * t17261 * t5386 + F::cast_from(0.42874018118069736972e-3_f64) * t5384 * t17265;
    t17268
}
