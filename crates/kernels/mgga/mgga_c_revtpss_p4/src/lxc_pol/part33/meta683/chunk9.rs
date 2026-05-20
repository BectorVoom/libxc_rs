//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2249/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2249<F: Float>(t20850: F, t2138: F, t29086: F, t5362: F, t104703: F, t104863: F, t104872: F, t104916: F, t104946: F, t112220: F, t1238: F, t1791: F, t20858: F, t20952: F, t21042: F, t21310: F, t26870: F, t29097: F, t3767: F, t5320: F, t5343: F, t5354: F, t97179: F) -> F {
    let t112373 = t20850 * t2138;
    let t112380 = t29086 * t5362;
    let t112395 = -F::cast_from(0.42874018118069736972e-3_f64) * t112373 * t1238 - F::cast_from(0.85748036236139473944e-3_f64) * t104916 * t1791 - F::cast_from(0.85748036236139473944e-3_f64) * t29086 * t5320 - F::cast_from(0.57165357490759649296e-3_f64) * t112380 + F::cast_from(0.85748036236139473944e-3_f64) * t97179 * t20858 - t104863 + F::cast_from(0.17149607247227894789e-2_f64) * t29097 * t20952 - F::cast_from(0.42874018118069736972e-3_f64) * t26870 * t21042 - F::cast_from(0.11433071498151929859e-2_f64) * t104946 * t21310 - F::cast_from(0.85748036236139473944e-3_f64) * t104703 * t5354 - F::cast_from(0.91464571985215438872e-2_f64) * t3767 * t112220 * t5343 + t104872;
    t112395
}
