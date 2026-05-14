//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1159/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1159<F: Float>(t1664: F, t5686: F, t649: F, t378: F, t5431: F, t735: F, t15: F, t2: F, t3: F, t4: F, t21073: F, t612: F, t1945: F, t424: F, t5457: F, t5456: F, t5865: F, t713: F) -> (F, F, F, F, F, F) {
    let t21136 = 0.13746876075482378975e2 * t649 * t5686 * t1664;
    let t21139 = 0.21687162600603479684e-1 * t735 * t378 * t5431;
    let t21145 = 1.0 / t15 / t2 / t4 / t3 / t378 / 48.0;
    let t21149 = 0.19758993022222222222e-1 * t612 * t21145 * t3 * t21073;
    let t21151 = t424 * t1945 * t5457;
    let t21155 = 0.42107210082969452692e2 * t5865 * t713 * t5456;
    (t21136, t21139, t21145, t21149, t21151, t21155)
}
