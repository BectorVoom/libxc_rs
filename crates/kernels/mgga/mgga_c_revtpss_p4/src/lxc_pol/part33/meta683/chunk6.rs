//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2246/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2246<F: Float>(t1219: F, t30800: F, t1241: F, t21100: F, t7616: F, t1256: F, t30789: F, t104770: F, t1230: F, t1252: F, t20802: F, t21095: F, t21300: F, t21334: F, t2138: F, t26870: F, t29040: F, t29097: F, t30815: F, t484: F, t5261: F, t6619: F, t8184: F, t97177: F, t97250: F) -> F {
    let t112301 = t30800 * t1219;
    let t112307 = t1241 * t7616 * t21100;
    let t112322 = t30789 * t1256;
    let t112327 = -t104770 + F::new(11.0) / F::new(324.0) * t112301 - F::cast_from(0.42874018118069736972e-3_f64) * t26870 * t21300 + t97177 / F::new(1296.0) + F::cast_from(0.14481890564325777821e-1_f64) * t112307 * t1252 + F::cast_from(0.57165357490759649296e-3_f64) * t97250 * t6619 - F::cast_from(0.57165357490759649296e-3_f64) * t29040 * t21095 + F::cast_from(0.85748036236139473944e-3_f64) * t29097 * t20802 - F::cast_from(0.45732285992607719436e-2_f64) * t5261 * t8184 * t484 + F::cast_from(0.14481890564325777821e-1_f64) * t1230 * t30815 * t484 + F::cast_from(0.28582678745379824648e-3_f64) * t112322 + F::cast_from(0.42874018118069736972e-3_f64) * t21334 * t2138 * t484;
    t112327
}
