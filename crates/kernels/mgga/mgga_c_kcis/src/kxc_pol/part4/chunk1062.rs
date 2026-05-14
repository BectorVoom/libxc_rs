//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1062/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1062<F: Float>(t13467: F, t15227: F, t14496: F, t421: F, t13516: F, t1662: F, t3611: F, t3515: F, t1259: F, t4951: F, t1262: F, t4621: F, t13475: F, t5310: F, t1071: F, t1851: F) -> (F, F, F, F, F, F) {
    let t15228 = t15227 * t13467;
    let t15231 = t14496 * t421;
    let t15232 = t15231 * t13516;
    let t15235 = t1662 * t3611;
    let t15236 = t3515 * t15235;
    let t15239 = t4951 * t1259;
    let t15240 = t4621 * t1262;
    let t15241 = t15239 * t15240;
    let t15244 = t5310 * t13475;
    let t15247 = t1851 * t1071;
    (t15228, t15232, t15236, t15241, t15244, t15247)
}
