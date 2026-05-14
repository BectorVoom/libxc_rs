//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 732/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk732<F: Float>(t2046: F, t7297: F, t8490: F, t1686: F, t2039: F, t270: F, t638: F, t1692: F, t1338: F, t535: F, t2050: F, t31: F, t2131: F, t5321: F, t38350: F, t7473: F) -> (F, F, F, F, F, F, F) {
    let t39792 = t2046 * t7297 * t8490;
    let t39796 = t638 * t2039 * t1686 * t270;
    let t39797 = 0.30487649791575028314e-3 * t39796;
    let t39800 = t638 * t2039 * t1692 * t270;
    let t39801 = 0.30487649791575028314e-3 * t39800;
    let t39804 = t638 * t2039 * t535 * t1338;
    let t39808 = t2046 * t2050 * t1686 * t31;
    let t39809 = 0.43368970657079495312e-4 * t39808;
    let t39827 = 0.4726e1 * t5321 * t2131;
    let t39832 = t38350 * t7473;
    (t39792, t39797, t39801, t39804, t39809, t39827, t39832)
}
