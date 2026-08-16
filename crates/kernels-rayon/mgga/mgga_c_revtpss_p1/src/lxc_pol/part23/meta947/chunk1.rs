//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3129/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3129(t24611: f64, t3172: f64, t3711: f64, t1042: f64, t1261: f64, t17202: f64, t17344: f64, t1789: f64, t20703: f64, t20982: f64, t21095: f64, t21203: f64, t5299: f64, t5381: f64, t56254: f64, t69668: f64, t69674: f64, t69698: f64, t69700: f64, t69795: f64, t78785: f64, t78790: f64) -> f64 {
    let t82351 = t3711 * t3172 * t24611;
    let t82367 = -0.25724410870841842183e-2_f64 * t1261 * t1042 * t17202 * t78790 - 0.34299214494455789578e-2_f64 * t1261 * t1042 * t56254 * t78785 + 0.57165357490759649296e-3_f64 * t82351 - 0.17149607247227894789e-2_f64 * t5381 * t20982 + 0.14481890564325777821e-1_f64 * t69795 * t5299 + 0.45732285992607719436e-2_f64 * t21203 * t21095 - 0.38586616306262763276e-2_f64 * t17344 * t1042 * t1789 * t20703 - 0.14291339372689912324e-3_f64 * t69668 + 0.14481890564325777821e-1_f64 * t69674 - 0.57165357490759649296e-3_f64 * t69698 - 0.28582678745379824648e-3_f64 * t69700;
    t82367
}
