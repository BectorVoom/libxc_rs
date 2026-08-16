//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 890/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk890<F: Float>(t16693: F, t16694: F, t4170: F, t16692: F, t1307: F, t6037: F, t4162: F, t4160: F, t11862: F, t5645: F, t5650: F, t5656: F) -> (F, F, F, F, F, F, F) {
    let t16695 = t16693 * t16694;
    let t16696 = t4170 * t16695;
    let t16697 = t16692 * t16696;
    let t16700 = t6037 * t1307;
    let t16701 = t4162 * t16700;
    let t16702 = t4160 * t16701;
    let t16704 = t11862 * t5645;
    let t16706 = t11862 * t5650;
    let t16708 = t11862 * t5656;
    (t16695, t16697, t16700, t16702, t16704, t16706, t16708)
}
