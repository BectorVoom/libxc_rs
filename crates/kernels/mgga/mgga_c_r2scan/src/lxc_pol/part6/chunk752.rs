//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 752/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk752<F: Float>(t1510: F, t406: F, t4959: F, t86: F, t1512: F, t410: F, t1411: F, t732: F, t1376: F, t457: F, t41: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5021 = t406 * t1510;
    let t5022 = 12.0 * t5021;
    let t5023 = t4959 * t86;
    let t5024 = 0.19751673498613801407e-1 * t5023;
    let t5025 = t406 * t1512;
    let t5026 = 24.0 * t5025;
    let t5027 = t410 * t1512;
    let t5028 = 24.0 * t5027;
    let t5029 = t732 * t1411;
    let t5030 = 0.17544670867903938621e1 * t5029;
    let t5031 = t1376 * t457;
    let t5032 = t41 * t5031;
    (t5021, t5022, t5024, t5026, t5027, t5028, t5029, t5030, t5031, t5032)
}
