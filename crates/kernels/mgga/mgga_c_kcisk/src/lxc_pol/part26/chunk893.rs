//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 893/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk893<F: Float>(t19950: F, t492: F, t14374: F, t79: F, t14364: F, t469: F, t12825: F, t451: F, t3598: F, t476: F, t2250: F, t979: F, t4265: F, t6300: F, t442: F, t5864: F) -> (F, F, F, F, F, F, F, F) {
    let t21011 = t19950 * t492;
    let t21029 = t79 * t14374;
    let t21050 = t14364 * t469;
    let t21113 = t12825 * t451;
    let t21145 = t476 * t3598;
    let t21152 = t979 * t2250;
    let t21154 = t4265 * t6300;
    let t21156 = t5864 * t442;
    (t21011, t21029, t21050, t21113, t21145, t21152, t21154, t21156)
}
