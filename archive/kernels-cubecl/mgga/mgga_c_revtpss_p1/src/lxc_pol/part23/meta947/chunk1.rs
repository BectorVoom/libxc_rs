//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3129/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3129<F: Float>(t24611: F, t3172: F, t3711: F, t1042: F, t1261: F, t17202: F, t17344: F, t1789: F, t20703: F, t20982: F, t21095: F, t21203: F, t5299: F, t5381: F, t56254: F, t69668: F, t69674: F, t69698: F, t69700: F, t69795: F, t78785: F, t78790: F) -> F {
    let t82351 = t3711 * t3172 * t24611;
    let t82367 = -F::cast_from(0.25724410870841842183e-2_f64) * t1261 * t1042 * t17202 * t78790 - F::cast_from(0.34299214494455789578e-2_f64) * t1261 * t1042 * t56254 * t78785 + F::cast_from(0.57165357490759649296e-3_f64) * t82351 - F::cast_from(0.17149607247227894789e-2_f64) * t5381 * t20982 + F::cast_from(0.14481890564325777821e-1_f64) * t69795 * t5299 + F::cast_from(0.45732285992607719436e-2_f64) * t21203 * t21095 - F::cast_from(0.38586616306262763276e-2_f64) * t17344 * t1042 * t1789 * t20703 - F::cast_from(0.14291339372689912324e-3_f64) * t69668 + F::cast_from(0.14481890564325777821e-1_f64) * t69674 - F::cast_from(0.57165357490759649296e-3_f64) * t69698 - F::cast_from(0.28582678745379824648e-3_f64) * t69700;
    t82367
}
