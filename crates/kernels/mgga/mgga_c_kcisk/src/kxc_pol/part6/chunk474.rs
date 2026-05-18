//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 474/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk474<F: Float>(t1457: F, t503: F, t475: F, t1486: F, t469: F, t382: F, t41: F, t3783: F, t484: F, t492: F, t497: F, t1414: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t4169 = F::new(1.0) / t1457 / t503;
    let t4170 = t475 * t4169;
    let t4203 = t1486 * t469;
    let t4204 = t41 * t382;
    let t4208 = t484 * t3783;
    let t4209 = t4208 * sigma0;
    let t4229 = t492 * t497;
    let t4230 = t1414 * t4229;
    (t4169, t4170, t4203, t4204, t4208, t4209, t4229, t4230)
}
