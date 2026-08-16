//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1004/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1004<F: Float>(t1983: F, t23857: F, t8640: F, t22949: F, t8607: F, t22584: F, t31758: F, t31035: F, t7217: F, t22597: F, t12734: F, t8533: F) -> (F, F, F, F, F, F) {
    let t115684 = F::cast_from(2.0_f64) * t1983 * t8640 * t23857;
    let t115690 = t8607 * t22949;
    let t115695 = F::cast_from(3.0_f64) * t1983 * t31758 * t22584;
    let t115698 = F::cast_from(2.0_f64) * t1983 * t7217 * t31035;
    let t115700 = F::cast_from(6.0_f64) * t8607 * t22597;
    let t115702 = F::cast_from(4.0_f64) * t12734 * t8533;
    (t115684, t115690, t115695, t115698, t115700, t115702)
}
