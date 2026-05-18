//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1364/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1364<F: Float>(t1005: F, t11124: F, t11127: F, t11130: F, t11133: F, t11149: F, t1434: F, t21616: F, t21670: F, t21672: F, t2539: F, t25624: F, t25627: F, t25651: F, t25654: F, t2577: F, t2578: F, t2593: F, t2599: F, t3565: F, t3584: F, t4283: F, t4311: F, t4324: F, t4327: F, t6993: F, t7104: F, t7109: F, t7133: F, t9048: F, t9051: F, t9054: F, t9058: F, t9061: F, t9065: F, t9195: F, t9248: F, t9255: F) -> F {
    let t29741 = F::new(0.19964560303604640732e6) * t21670 * t4283 * t21672 * t2539 - F::new(0.46785788981077169656e1) * t25624 * t3565 - F::new(0.46785788981077169656e1) * t9255 * t9048 - F::new(0.23392894490538584828e1) * t9255 * t9051 - F::new(0.2077903092681775651e3) * t25627 * t9054 + F::new(0.69263436422725855034e2) * t25651 * t3584 + F::new(0.69263436422725855034e2) * t9248 * t9058 + F::new(0.34631718211362927517e2) * t9248 * t9061 + F::new(0.20508037716432813315e4) * t25654 * t9065 + F::new(0.70178683471615754484e1) * t7104 * t11124 + F::new(0.35089341735807877242e1) * t2599 * t4311 * t2593 + F::new(0.6233709278045326953e3) * t6993 * t4327 * t2578 - F::new(0.46785788981077169656e1) * t7133 * t11127 - F::new(0.23392894490538584828e1) * t2577 * t1434 * t9195 - F::new(0.20779030926817756511e3) * t21616 * t11130 - F::new(0.10389515463408878255e3) * t7109 * t4327 * t2593 - F::new(0.23392894490538584828e1) * t7133 * t11133 - F::new(0.23392894490538584828e1) * t2577 * t11149 * t1005 - F::new(0.11696447245269292414e1) * t2577 * t4324 * t2593;
    t29741
}
