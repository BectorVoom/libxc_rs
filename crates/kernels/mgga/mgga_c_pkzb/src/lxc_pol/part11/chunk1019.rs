//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1019/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1019<F: Float>(t1184: F, t3739: F, t852: F, t2240: F, t1185: F, t9976: F, t3033: F, t3766: F, t3769: F, t8219: F, t2242: F, t6142: F) -> (F, F, F, F, F, F, F, F) {
    let t11233 = t3739 * t1184;
    let t11234 = t11233 * t852;
    let t11236 = F::new(6.0) * t2240 * t11234;
    let t11238 = F::new(3.0) * t9976 * t1185;
    let t11240 = F::new(3.0) * t3033 * t3766;
    let t11242 = F::cast_from(0.48245938496077605201e2_f64) * t8219 * t3769;
    let t11243 = t11233 * t2242;
    let t11245 = F::cast_from(0.96491876992155210402e2_f64) * t6142 * t11243;
    (t11233, t11234, t11236, t11238, t11240, t11242, t11243, t11245)
}
