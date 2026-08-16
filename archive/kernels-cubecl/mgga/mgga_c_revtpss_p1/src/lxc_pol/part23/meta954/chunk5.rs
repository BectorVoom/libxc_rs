//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3178/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3178<F: Float>(t17572: F, t21188: F, t1042: F, t1214: F, t1261: F, t17235: F, t20792: F, t21272: F, t22671: F, t3711: F, t5270: F, t5279: F, t5296: F, t5391: F, t57136: F, t69795: F, t70664: F, t70667: F, t70672: F, t78785: F, t78790: F, t82543: F) -> F {
    let t83462 = t17572 * t21188;
    let t83480 = -F::cast_from(0.85748036236139473944e-3_f64) * t70664 + F::cast_from(0.42874018118069736972e-3_f64) * t70667 + F::cast_from(0.57165357490759649296e-3_f64) * t70672 - F::cast_from(0.3811023832717309953e-2_f64) * t5391 * t20792 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t1042 * t5296 * t22671 * t1214 + F::cast_from(0.85748036236139473947e-3_f64) * t83462 + F::cast_from(0.14481890564325777821e-1_f64) * t69795 * t5279 + F::cast_from(0.63517063878621832552e-3_f64) * t3711 * t1042 * t17235 * t82543 - F::cast_from(0.28963781128651555642e-1_f64) * t21272 * t5270 - F::cast_from(0.19055119163586549766e-2_f64) * t1261 * t1042 * t17235 * t78790 - F::cast_from(0.76220476654346199062e-2_f64) * t1261 * t1042 * t57136 * t78785;
    t83480
}
