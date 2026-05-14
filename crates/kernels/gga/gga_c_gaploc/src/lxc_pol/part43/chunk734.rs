//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 734/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk734<F: Float>(t40172: F, t40176: F, t40178: F, t40182: F, t40187: F, t10557: F, t9324: F, t30829: F, t31769: F, t544: F, t913: F, t1424: F, t2875: F, t9060: F, t40202: F, t3177: F, t8272: F, t9267: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t41849 = 0.19171462976960374838e1 * t40172;
    let t41850 = 0.42603251059911944084e0 * t40176;
    let t41851 = 0.11502877786176224903e1 * t40178;
    let t41852 = 0.25561950635947166451e0 * t40182;
    let t41854 = 0.17875244975925213335e0 * t40187;
    let t41874 = 0.85801175884441024006e1 * t10557 * t9324;
    let t41884 = t544 * t30829 * t913 * t31769;
    let t41885 = 0.3575048995185042667e0 * t41884;
    let t41889 = 0.39722766613167140743e-1 * t544 * t9060 * t2875 * t1424;
    let t41893 = 0.46011511144704899612e1 * t40202;
    let t41903 = t9267 * t8272 * t3177;
    (t41849, t41850, t41851, t41852, t41854, t41874, t41885, t41889, t41893, t41903)
}
