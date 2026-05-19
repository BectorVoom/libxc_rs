//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 926/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk926<F: Float>(t1532: F, t7046: F, t5325: F, t5328: F, t5339: F, t5025: F, t5028: F, t5040: F, t5066: F, t5069: F, t5073: F, t5186: F, t5324: F, t5333: F, t5338: F, t5344: F, t7045: F) -> (F, F, F, F, F) {
    let t7047 = t7046 * t1532;
    let t7048 = F::cast_from(0.10843581300301739842e-1_f64) * t7047;
    let t7049 = F::cast_from(0.4883052614935078681e-3_f64) * t5325;
    let t7050 = F::cast_from(0.18311447306006545054e-3_f64) * t5328;
    let t7051 = F::cast_from(0.11696447245269292414e1_f64) * t5339;
    let t7052 = t5186 + t7045 + t5025 + t7048 + t5028 - t5324 + t5040 + t5066 - t5069 - t5073 + t7049 - t7050 + t5333 - t5338 - t7051 - t5344;
    (t7048, t7049, t7050, t7051, t7052)
}
