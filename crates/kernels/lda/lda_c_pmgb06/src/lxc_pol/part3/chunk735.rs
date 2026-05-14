//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 735/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk735<F: Float>(t107: F, t110: F, t122: F, t202: F, t4060: F, t4063: F, t4174: F, t4177: F, t4181: F, t4185: F, t4575: F, t5508: F, t5514: F, t5517: F, t5518: F, t5520: F, t5524: F, t5526: F, t5529: F) -> (F,) {
    let t5538 = -0.011938374665504766 * t122 * t202 * t5508 + t5514 - t5517 - 0.1675256410710088 * t5518 - 0.1675256410710088 * t5520 + t5524 - 0.053059442957798957 * t5526 + 1.328721022894618 * t5529 - 0.10611888591559791 * t4174 + 0.019897291109174608 * t4177 - t4181 + t4185 + 2.657442045789236 * t4063 - 0.5694518669548363 * t4060 + 0.42708890021612717 * t107 * t110 * t4575;
    (t5538,)
}
