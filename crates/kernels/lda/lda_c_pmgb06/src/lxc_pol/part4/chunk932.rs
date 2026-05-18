//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 932/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk932<F: Float>(t107: F, t2407: F, t410: F, t122: F, t2659: F, t569: F, t110: F, t202: F, t4063: F, t4174: F, t4181: F, t4185: F, t5514: F, t5517: F, t5518: F, t5520: F, t5524: F, t5526: F, t5529: F, t6104: F, t6913: F) -> F {
    let t6918 = t107 * t410 * t2407;
    let t6922 = t122 * t569 * t2659;
    let t6927 = t5514 + F::new(0.42708890021612717) * t107 * t110 * t6104 - t5517 - F::new(0.3350512821420176) * t5518 - F::new(0.3350512821420176) * t5520 + t5524 - F::new(0.011938374665504766) * t122 * t202 * t6913 - F::new(0.5694518669548363) * t6918 - F::new(0.10611888591559791) * t5526 + F::new(0.019897291109174608) * t6922 + F::new(2.657442045789236) * t5529 - F::new(0.053059442957798957) * t4174 - t4181 + t4185 + F::new(1.328721022894618) * t4063;
    t6927
}
