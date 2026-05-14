//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 734/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk734<F: Float>(t1614: F, t51: F, t213: F, t1109: F, t679: F, t689: F, t1127: F, t709: F, t39: F, t695: F, t224: F, t3781: F, t7853: F, t1160: F, t2486: F, t1526: F, t5198: F, t9483: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17839 = t51 * t1614;
    let t17840 = t17839 * t213;
    let t17841 = t1109 * t679;
    let t17842 = t17841 * t689;
    let t17843 = t17840 * t17842;
    let t17859 = t1127 * t709;
    let t17863 = t1127 * t679;
    let t17864 = t17863 * t689;
    let t17986 = t695 * t39;
    let t17987 = t224 * t17986;
    let t17994 = t7853 * t3781;
    let t18467 = t2486 * t1160;
    let t18959 = t1526 * t9483 * t5198;
    (t17839, t17840, t17841, t17843, t17859, t17863, t17864, t17987, t17994, t18467, t18959)
}
