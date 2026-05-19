//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 790/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk790<F: Float>(t1066: F, t179: F, t5672: F, t299: F, t2939: F, t771: F, t2068: F, t2739: F, t2099: F, t2947: F, t2945: F, t2003: F) -> (F, F, F, F, F, F) {
    let t7755 = t179 * t5672 * t1066;
    let t7756 = t299 * t7755;
    let t7760 = t771 * t2939;
    let t7765 = t179 * t2068 * t2739;
    let t7767 = F::cast_from(0.57165357490759649296e-3_f64) * t299 * t7765;
    let t7784 = t2099 * t2947;
    let t7786 = F::cast_from(0.17149607247227894789e-2_f64) * t2945 * t7784;
    let t7787 = t2003 * t2739;
    (t7755, t7756, t7760, t7767, t7786, t7787)
}
