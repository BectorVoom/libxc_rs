//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 796/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk796<F: Float>(t199: F, t5522: F, t122: F, t1669: F, t886: F, t107: F, t1180: F, t902: F, t110: F, t202: F, t4060: F, t4063: F, t4174: F, t4177: F, t4181: F, t4185: F, t4575: F, t5508: F, t5514: F, t5517: F, t5518: F, t5520: F) -> F {
    let t5524 = F::cast_from(0.1675256410710088_f64) * t5522 * t199;
    let t5526 = t122 * t1669 * t886;
    let t5529 = t107 * t1180 * t902;
    let t5538 = -F::cast_from(0.011938374665504766_f64) * t122 * t202 * t5508 + t5514 - t5517 - F::cast_from(0.1675256410710088_f64) * t5518 - F::cast_from(0.1675256410710088_f64) * t5520 + t5524 - F::cast_from(0.053059442957798957_f64) * t5526 + F::cast_from(1.328721022894618_f64) * t5529 - F::cast_from(0.10611888591559791_f64) * t4174 + F::cast_from(0.019897291109174608_f64) * t4177 - t4181 + t4185 + F::cast_from(2.657442045789236_f64) * t4063 - F::cast_from(0.5694518669548363_f64) * t4060 + F::cast_from(0.42708890021612717_f64) * t107 * t110 * t4575;
    t5538
}
