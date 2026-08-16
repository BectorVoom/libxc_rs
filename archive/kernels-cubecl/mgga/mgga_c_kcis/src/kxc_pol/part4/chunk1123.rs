//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1123/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1123<F: Float>(t330: F, t4625: F, t829: F, t3269: F, t2635: F, t4595: F, t1670: F, t2844: F, t10292: F, t2630: F, t313: F, t4600: F) -> (F, F, F, F) {
    let t14182 = t4625 * t330;
    let t14183 = t14182 * t829;
    let t14184 = t3269 * t14183;
    let t14188 = t3269 * t4595 * t2635;
    let t14191 = t1670 * t2844;
    let t14193 = t10292 * t14191 * t2630;
    let t14196 = t4600 * t313;
    (t14184, t14188, t14193, t14196)
}
