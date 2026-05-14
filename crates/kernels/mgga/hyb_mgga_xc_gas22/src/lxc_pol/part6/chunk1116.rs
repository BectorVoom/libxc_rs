//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1116/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1116<F: Float>(t14: F, t20685: F, t237: F, t1031: F, t1047: F, t21864: F, t21866: F, t21869: F, t21872: F, t21875: F, t21880: F, t21882: F, t21884: F, t21886: F, t2679: F, t576: F) -> (F, F, F) {
    let t21889 = t237 * t14 * t20685;
    let t21894 = 1.0 * t1031 * (-0.21099166666666666667e1 * t21864 + 0.202552e2 * t21866 - 0.75019259259259259258e1 * t21869 + 0.6564185185185185185e1 * t21872 + 0.31003950617283950618e1 * t21875 + 0.68258333333333333335e-1 * t21880 - 0.10921333333333333333e1 * t21882 + 0.12134814814814814815e1 * t21884 + 0.10617962962962962963e1 * t21886 + 0.13388493827160493828e1 * t21889) * t1047;
    let t21896 = 1.0 / t2679 / t576;
    (t21889, t21894, t21896)
}
