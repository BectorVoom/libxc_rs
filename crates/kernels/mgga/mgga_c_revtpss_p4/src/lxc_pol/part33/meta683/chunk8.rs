//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2248/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2248<F: Float>(t30799: F, t800: F, t21270: F, t2137: F, t467: F, t20926: F, t26870: F, t104647: F, t104752: F, t104844: F, t104924: F, t1227: F, t1266: F, t1791: F, t17934: F, t20838: F, t20923: F, t20934: F, t29062: F, t29096: F, t29100: F, t5279: F, t5320: F, t5343: F, t6611: F, t97174: F, t97292: F) -> F {
    let t112350 = t30799 * t800;
    let t112356 = t467 * t2137 * t21270;
    let t112364 = t26870 * t20926;
    let t112372 = F::cast_from(0.57165357490759649296e-3_f64) * t104752 * t5279 + t104844 - F::cast_from(0.95275595817932748827e-3_f64) * t104647 * t20923 - F::new(11.0) / F::new(324.0) * t112350 * t1227 + F::cast_from(0.57165357490759649296e-3_f64) * t97174 * t20934 - F::cast_from(0.96545937095505185473e-2_f64) * t112356 * t1266 - F::cast_from(0.85748036236139473944e-3_f64) * t29100 * t20838 + F::cast_from(0.17149607247227894789e-2_f64) * t17934 * t29096 * t5343 - F::cast_from(0.57165357490759649296e-3_f64) * t112364 + F::cast_from(0.85748036236139473944e-3_f64) * t97292 * t6611 + F::cast_from(0.45732285992607719436e-2_f64) * t104924 * t1791 + F::cast_from(0.45732285992607719436e-2_f64) * t29062 * t5320;
    t112372
}
