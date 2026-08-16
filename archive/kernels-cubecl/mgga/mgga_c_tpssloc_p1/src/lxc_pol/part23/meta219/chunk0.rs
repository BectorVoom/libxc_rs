//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 866/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk866<F: Float>(t1667: F, t2403: F, t1657: F, t3263: F, t3312: F, t1720: F, t3030: F, t3609: F, t1687: F, t3400: F, t3375: F, t1675: F, t3356: F) -> (F, F, F, F, F, F, F, F) {
    let t14766 = t2403 * t1667;
    let t14838 = t1657 * t3263;
    let t14850 = t1657 * t3312;
    let t15026 = t1720 * t3030;
    let t15027 = t15026 * t3609;
    let t15126 = t1687 * t3400;
    let t15136 = t1687 * t3375;
    let t15146 = t1675 * t3356;
    (t14766, t14838, t14850, t15026, t15027, t15126, t15136, t15146)
}
