//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1051/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1051<F: Float>(t11535: F, t11541: F, t11550: F, t11555: F, t11559: F, t11562: F, t11592: F, t11595: F, t12129: F, t12131: F, t12135: F, t12136: F, t12137: F, t12138: F, t12139: F, t12140: F, t12141: F, t12144: F, t12145: F, t12146: F) -> F {
    let t12147 = F::cast_from(0.49166375783284505217e-7_f64) * t11535 - F::cast_from(0.22099262292595577329e-7_f64) * t11541 + t12129 - F::cast_from(0.252977417353824213e-7_f64) * t11550 - t12131 - F::cast_from(0.49166375783284505217e-8_f64) * t11555 + F::cast_from(0.32777583855523003478e-8_f64) * t11559 - F::cast_from(0.57970906942607043474e-5_f64) * t11562 + t12135 + t12136 - t12137 + t12138 - t12139 + t12140 - t12141 + F::cast_from(0.96684272530105650816e-8_f64) * t11592 + F::cast_from(0.90579542097823505425e-7_f64) * t11595 + t12144 + t12145 - t12146;
    t12147
}
