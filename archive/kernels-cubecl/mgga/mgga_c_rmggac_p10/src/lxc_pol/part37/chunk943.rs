//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 943/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk943<F: Float>(t77080: F, t74536: F, t74539: F, t74549: F, t14451: F, t1627: F, t26287: F, t8377: F, t30204: F, t1632: F, t1635: F, t26283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77081 = F::cast_from(0.34093327067806677161e-2_f64) * t77080;
    let t77082 = F::cast_from(0.1276937996798935182e-4_f64) * t74536;
    let t77083 = F::cast_from(0.1276937996798935182e-4_f64) * t74539;
    let t77084 = F::cast_from(0.15961724959986689775e-4_f64) * t74549;
    let t77085 = t14451 * t1627;
    let t77086 = t26287 * t77085;
    let t77087 = F::cast_from(0.8980681276397856423e-1_f64) * t77086;
    let t77088 = t14451 * t8377;
    let t77089 = t30204 * t77088;
    let t77090 = F::cast_from(0.5987120850931904282e-1_f64) * t77089;
    let t77091 = t14451 * t1632;
    let t77092 = t26287 * t77091;
    let t77093 = F::cast_from(0.8980681276397856423e-1_f64) * t77092;
    let t77094 = t14451 * t1635;
    let t77095 = t26283 * t77094;
    (t77081, t77082, t77083, t77084, t77085, t77087, t77088, t77090, t77091, t77093, t77094, t77095)
}
