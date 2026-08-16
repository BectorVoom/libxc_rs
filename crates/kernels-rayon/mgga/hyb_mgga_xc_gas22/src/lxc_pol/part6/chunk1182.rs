//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1182/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1182(t1056: f64, t7349: f64, t21868: f64, t2666: f64, t1040: f64, t21871: f64, t14: f64, t20685: f64, t237: f64, t1031: f64, t1047: f64, t21864: f64, t21866: f64, t21869: f64, t21872: f64, t21875: f64, t21880: f64) -> (f64, f64, f64, f64, f64) {
    let t21882 = t7349 * t1056;
    let t21884 = t2666 * t21868;
    let t21886 = t1040 * t21871;
    let t21889 = t237 * t14 * t20685;
    let t21894 = 1.0_f64 * t1031 * (-0.21099166666666666667e1_f64 * t21864 + 0.202552e2_f64 * t21866 - 0.75019259259259259258e1_f64 * t21869 + 0.6564185185185185185e1_f64 * t21872 + 0.31003950617283950618e1_f64 * t21875 + 0.68258333333333333335e-1_f64 * t21880 - 0.10921333333333333333e1_f64 * t21882 + 0.12134814814814814815e1_f64 * t21884 + 0.10617962962962962963e1_f64 * t21886 + 0.13388493827160493828e1_f64 * t21889) * t1047;
    (t21882, t21884, t21886, t21889, t21894)
}
