//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1968/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1968<F: Float>(t102293: F, t102296: F, t102298: F, t102306: F, t102309: F, t1444: F, t1882: F, t25921: F, t25924: F, t26333: F, t26351: F, t27837: F, t28815: F, t28840: F, t543: F, t7295: F, t7301: F, t96284: F, t96287: F, t96289: F, t96292: F, t96294: F) -> F {
    let t102313 = F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t26333 * t1882 * t543 - F::cast_from(0.52041769129231196772e1_f64) * t25921 * t28815 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t26351 - F::cast_from(0.34270468708064099208e-1_f64) * t102293 - F::cast_from(0.96373646535613327357e-2_f64) * t102296 - t96284 + F::cast_from(0.17135234354032049604e-1_f64) * t102298 - F::cast_from(0.52041769129231196772e1_f64) * t7295 * t25924 * t28840 * t1444 + t102306 - F::cast_from(0.45699670022203476294e-2_f64) * t96287 + t102309 + F::cast_from(0.34270468708064099208e-1_f64) * t96289 + F::cast_from(0.14456046980341999104e-1_f64) * t96292 - F::cast_from(0.25702851531048074406e-1_f64) * t96294;
    t102313
}
