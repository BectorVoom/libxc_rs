//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2164/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2164<F: Float>(t2225: F, t5166: F, t15921: F, t592: F, t17: F, t2516: F, t5151: F, t1787: F, t9861: F, t15971: F, t2221: F, t5168: F) -> (F, F, F, F, F, F) {
    let t54400 = t2225 * t5166;
    let t54401 = F::cast_from(60.0_f64) * t54400;
    let t54402 = t592 * t15921;
    let t54403 = F::cast_from(24.0_f64) * t54402;
    let t54408 = t17 * t5151 * t2516;
    let t54409 = F::cast_from(3.0_f64) * t54408;
    let t54411 = t17 * t1787 * t9861;
    let t54412 = t592 * t15971;
    let t54428 = t2221 * t5168;
    (t54401, t54403, t54409, t54411, t54412, t54428)
}
