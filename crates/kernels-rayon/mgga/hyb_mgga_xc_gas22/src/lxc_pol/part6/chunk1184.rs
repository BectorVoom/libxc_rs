//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1184/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1184(t21864: f64, t21866: f64, t21869: f64, t21872: f64, t21875: f64, t21880: f64, t21882: f64, t21884: f64, t21886: f64, t21889: f64, t1089: f64, t1096: f64, t1110: f64) -> (f64, f64) {
    let t21951 = -0.28769444444444444444e1_f64 * t21864 + 0.27618666666666666667e2_f64 * t21866 - 0.10229135802469135803e2_f64 * t21869 + 0.89504938271604938273e1_f64 * t21872 + 0.31310740740740740741e1_f64 * t21875 + 0.366775e-1_f64 * t21880 - 0.58684e0_f64 * t21882 + 0.65204444444444444445e0_f64 * t21884 + 0.5705388888888888889e0_f64 * t21886 + 0.13490888888888888889e1_f64 * t21889;
    let t21955 = 0.5848223622634646207e0_f64 * t1110 * t1089 * t21951 * t1096;
    (t21951, t21955)
}
