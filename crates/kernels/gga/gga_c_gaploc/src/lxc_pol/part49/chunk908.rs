//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 908/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk908<F: Float>(t44133: F, t1445: F, t3209: F, t833: F, t8469: F, t25405: F, t5748: F, t13034: F, t15751: F, t10948: F, t9972: F, t41451: F, t41454: F, t41457: F, t41460: F, t41463: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44134 = 0.15976219147466979032e-1 * t44133;
    let t44138 = 0.43710935587469654631e2 * t833 * t1445 * t8469 * t3209;
    let t44142 = 0.27606906686822939767e2 * t5748 * t1445 * t25405 * t3209;
    let t44144 = 0.27606906686822939767e2 * t15751 * t13034;
    let t44145 = t10948 * t9972;
    let t44148 = 0.89376224879626066674e-1 * t41451;
    let t44149 = 0.59584149919750711116e-1 * t41454;
    let t44150 = 0.15337170381568299871e1 * t41457;
    let t44151 = 0.3575048995185042667e0 * t41460;
    let t44152 = 0.17875244975925213335e0 * t41463;
    (t44134, t44138, t44142, t44144, t44145, t44148, t44149, t44150, t44151, t44152)
}
