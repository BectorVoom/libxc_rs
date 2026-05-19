//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 372/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk372<F: Float>(t1689: F, t1808: F, t1809: F, t1810: F, t1825: F, t1860: F, t604: F, t674: F, t702: F) -> F {
    let t1862 = -t1808 - F::cast_from(0.23426533963880895498e-2_f64) * t1809 * t1810 - F::cast_from(0.46853067927761790996e-2_f64) * t674 * t1825 - t1689 * t702 - t604 * t1860;
    t1862
}
