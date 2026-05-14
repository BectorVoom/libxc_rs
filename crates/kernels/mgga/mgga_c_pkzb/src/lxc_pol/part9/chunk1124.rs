//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1124/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1124<F: Float>(t178: F, t18016: F, t7707: F, t7710: F, t17933: F, t17930: F, t1123: F, t17938: F, t2030: F, t5726: F, t18000: F, t18002: F, t18009: F, t5955: F, t5984: F, t7637: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21448 = t18016 * t178;
    let t21452 = t7707 * t7710;
    let t21454 = t17933 * t178;
    let t21455 = t17930 * t21454;
    let t21456 = t1123 * t17938;
    let t21457 = t2030 * t5726;
    let t21462 = t18000 * t21454;
    let t21463 = t18002 * t5726;
    let t21468 = t18009 * t21454;
    let t21469 = t5955 * t5726;
    let t21485 = t5984 * t7637;
    (t21448, t21452, t21454, t21455, t21456, t21457, t21462, t21463, t21468, t21469, t21485)
}
