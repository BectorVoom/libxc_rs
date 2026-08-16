//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1160/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1160<F: Float>(t25162: F, t95296: F, t2047: F, t92576: F, t92584: F, t2247: F, t2251: F, t68: F, t26182: F, t6960: F, t92565: F, t92588: F, t95284: F, t95286: F, t95288: F, t95290: F, t95294: F) -> F {
    let t95297 = t25162 * t95296;
    let t95303 = t2047 * t92576;
    let t95306 = t2047 * t92584;
    let t95310 = t2247 * t2251 * t68;
    let t95313 = F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t95284 + F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t95286 + F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t95288 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t95290 - F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t95294 - F::cast_from(160.0_f64) / F::cast_from(3.0_f64) * t95297 + F::cast_from(20.0_f64) * t92565 * t26182 + F::cast_from(10.0_f64) * t92588 * t26182 + F::cast_from(20.0_f64) * t25162 * t95303 + F::cast_from(10.0_f64) * t25162 * t95306 + F::cast_from(10.0_f64) * t95310 * t6960;
    t95313
}
