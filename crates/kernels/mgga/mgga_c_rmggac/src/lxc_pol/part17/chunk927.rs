//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 927/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk927<F: Float>(t1614: F, t3351: F, t511: F, t618: F, t7231: F, t10095: F, t16043: F, t1528: F, t515: F, t570: F, t1652: F, t39388: F, t45403: F, t45407: F, t45411: F, t45415: F, t45420: F, t45424: F, t45428: F, t45432: F, t45436: F, t45439: F, t45441: F, t45446: F) -> F {
    let t45451 = t3351 * t7231 * t511 * t618 * t1614;
    let t45453 = t16043 * t10095;
    let t45458 = t3351 * t7231 * t515 * t1528 * t570;
    let t45463 = t3351 * t7231 * t515 * t618 * t1652;
    let t45465 = -F::new(0.1064114997332445985e-4) * t45403 + F::new(0.3192344991997337955e-4) * t45407 - F::new(0.3192344991997337955e-4) * t45411 - F::new(0.1064114997332445985e-4) * t45415 + F::new(0.29810146462873361018e-2) * t39388 - F::new(0.40911992481368012592e-1) * t45420 - F::new(0.212822999466489197e-4) * t45424 - F::new(0.17025839957319135759e-4) * t45428 + F::new(0.51077519871957407276e-4) * t45432 - F::new(0.17025839957319135759e-4) * t45436 + F::new(0.17025839957319135759e-4) * t45439 - F::new(0.31923449919973379548e-4) * t45441 + F::new(0.25538759935978703638e-4) * t45446 + F::new(0.25538759935978703638e-4) * t45451 + F::new(0.85129199786595678796e-5) * t45453 + F::new(0.85129199786595678796e-5) * t45458 + F::new(0.85129199786595678796e-5) * t45463;
    t45465
}
