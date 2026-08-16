//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 834/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk834<F: Float>(t2250: F, t751: F, t707: F, t2447: F, t706: F, t708: F, t157: F, t9448: F, t182: F, t2509: F, t746: F, t9490: F) -> (F, F, F, F, F, F, F) {
    let t9909 = t751 * t2250;
    let t9910 = t707 * t9909;
    let t9911 = F::cast_from(12.0_f64) * t9910;
    let t9912 = t706 * t2447;
    let t9914 = F::cast_from(12.0_f64) * t9912 * t708;
    let t9915 = t9448 * t157;
    let t9917 = F::cast_from(0.19751673498613801407e-1_f64) * t9915 * t182;
    let t9919 = t2509 * t9490 * t746;
    (t9909, t9911, t9912, t9914, t9915, t9917, t9919)
}
