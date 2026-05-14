//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 695/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk695<F: Float>(t1414: F, t9: F, t1035: F, t352: F, t273: F, t57: F, t1509: F, t424: F, t41: F, t1477: F, t1485: F, t1483: F, t400: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3436 = 1.0 / t9 / t1414;
    let t3675 = t352 * t1035;
    let t4145 = t57 * t273;
    let t4694 = t424 * t1509;
    let t4695 = t41 * t4694;
    let t4696 = 3.0 * t4695;
    let t4700 = t1477 * t1485;
    let t4702 = t1483 * t4700 * t400;
    let t4703 = 0.48245938496077605201e2 * t4702;
    (t3436, t3675, t4145, t4694, t4695, t4696, t4700, t4702, t4703)
}
