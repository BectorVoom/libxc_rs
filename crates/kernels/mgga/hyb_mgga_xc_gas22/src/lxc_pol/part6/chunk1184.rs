//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1184/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1184<F: Float>(t21864: F, t21866: F, t21869: F, t21872: F, t21875: F, t21880: F, t21882: F, t21884: F, t21886: F, t21889: F, t1089: F, t1096: F, t1110: F) -> (F, F) {
    let t21951 = -F::cast_from(0.28769444444444444444e1_f64) * t21864 + F::cast_from(0.27618666666666666667e2_f64) * t21866 - F::cast_from(0.10229135802469135803e2_f64) * t21869 + F::cast_from(0.89504938271604938273e1_f64) * t21872 + F::cast_from(0.31310740740740740741e1_f64) * t21875 + F::new(0.366775e-1) * t21880 - F::new(0.58684e0) * t21882 + F::cast_from(0.65204444444444444445e0_f64) * t21884 + F::cast_from(0.5705388888888888889e0_f64) * t21886 + F::cast_from(0.13490888888888888889e1_f64) * t21889;
    let t21955 = F::cast_from(0.5848223622634646207e0_f64) * t1110 * t1089 * t21951 * t1096;
    (t21951, t21955)
}
