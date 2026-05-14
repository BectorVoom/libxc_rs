//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 942/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk942<F: Float>(t15216: F, t4581: F, t3514: F, t421: F, t9959: F, t4567: F, t9897: F, t14496: F, t1259: F, t4951: F, t187: F, t4731: F, t1684: F, t3005: F, t3034: F, t4758: F) -> (F, F, F, F, F, F, F, F) {
    let t15217 = t15216 * t4581;
    let t15219 = t3514 * t15217 / 432.0;
    let t15220 = t9959 * t421;
    let t15221 = t15220 * t4567;
    let t15223 = t3514 * t15221 / 648.0;
    let t15227 = t9897 * t421;
    let t15231 = t14496 * t421;
    let t15239 = t4951 * t1259;
    let t15296 = t187 * t4731;
    let t15304 = t1684 * t3005;
    let t15351 = t4758 * t3034;
    (t15219, t15223, t15227, t15231, t15239, t15296, t15304, t15351)
}
