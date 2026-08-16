//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3178/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3178(t17572: f64, t21188: f64, t1042: f64, t1214: f64, t1261: f64, t17235: f64, t20792: f64, t21272: f64, t22671: f64, t3711: f64, t5270: f64, t5279: f64, t5296: f64, t5391: f64, t57136: f64, t69795: f64, t70664: f64, t70667: f64, t70672: f64, t78785: f64, t78790: f64, t82543: f64) -> f64 {
    let t83462 = t17572 * t21188;
    let t83480 = -0.85748036236139473944e-3_f64 * t70664 + 0.42874018118069736972e-3_f64 * t70667 + 0.57165357490759649296e-3_f64 * t70672 - 0.3811023832717309953e-2_f64 * t5391 * t20792 + 0.14291339372689912324e-3_f64 * t3711 * t1042 * t5296 * t22671 * t1214 + 0.85748036236139473947e-3_f64 * t83462 + 0.14481890564325777821e-1_f64 * t69795 * t5279 + 0.63517063878621832552e-3_f64 * t3711 * t1042 * t17235 * t82543 - 0.28963781128651555642e-1_f64 * t21272 * t5270 - 0.19055119163586549766e-2_f64 * t1261 * t1042 * t17235 * t78790 - 0.76220476654346199062e-2_f64 * t1261 * t1042 * t57136 * t78785;
    t83480
}
