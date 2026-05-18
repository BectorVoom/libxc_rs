//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 982/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk982<F: Float>(t1538: F, t7385: F, t571: F, t1551: F, t7328: F, t578: F, t5929: F, t6002: F, t1547: F, t1546: F, t21876: F, t6011: F) -> (F, F, F, F, F, F, F) {
    let t22419 = t7385 * t1538;
    let t22420 = t571 * t22419;
    let t22422 = t7328 * t1551;
    let t22423 = t578 * t22422;
    let t22425 = t6002 * t5929;
    let t22427 = t7328 * t1547;
    let t22428 = t1546 * t22427;
    let t22430 = t6011 * t21876;
    (t22420, t22422, t22423, t22425, t22427, t22428, t22430)
}
