//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1031/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1031<F: Float>(t2880: F, t421: F, t4581: F, t3514: F, t9959: F, t4567: F, t9897: F, t14496: F, t1259: F, t4951: F, t187: F, t4731: F) -> (F, F, F, F, F, F) {
    let t15216 = t2880 * t421;
    let t15217 = t15216 * t4581;
    let t15219 = t3514 * t15217 / F::new(432.0);
    let t15220 = t9959 * t421;
    let t15221 = t15220 * t4567;
    let t15223 = t3514 * t15221 / F::new(648.0);
    let t15227 = t9897 * t421;
    let t15231 = t14496 * t421;
    let t15239 = t4951 * t1259;
    let t15296 = t187 * t4731;
    (t15219, t15223, t15227, t15231, t15239, t15296)
}
