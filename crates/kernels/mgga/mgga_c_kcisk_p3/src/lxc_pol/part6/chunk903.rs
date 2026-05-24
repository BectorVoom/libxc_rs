//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 903/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk903<F: Float>(t604: F, t2464: F, t8518: F, t5015: F, t28532: F, t1783: F, t1310: F, t2430: F, t4957: F, t23450: F, t10949: F, t1224: F, t28369: F) -> (F, F, F, F) {
    let t659 = F::new(0.0) < t604;
    let t29054 = t8518 * t2464;
    let t29055 = t5015 * t29054;
    let t29059 = piecewise3::<F>(t659, t28532, -t28532);
    let t29060 = t1783 * t29059;
    let t29061 = t1310 * t29060;
    let t29073 = t4957 * t2430;
    let t29074 = t23450 * t29073;
    let t29082 = t1224 * t10949 * t28369;
    (t29055, t29061, t29074, t29082)
}
