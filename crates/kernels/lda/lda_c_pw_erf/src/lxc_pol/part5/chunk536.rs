//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 536/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk536<F: Float>(t1085: F, t2704: F, t1077: F, t156: F, t1084: F, t402: F, t474: F, t14: F, t25: F, t2: F, t39: F, t717: F, t732: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2705 = t2704 * t1085;
    let t2706 = F::cast_from(0.032530742648344574_f64) * t2705;
    let t2707 = t156 * t1077;
    let t2708 = t1084 * t2707;
    let t2709 = F::cast_from(0.032530742648344574_f64) * t2708;
    let t2710 = t474 * t402;
    let t2711 = t1084 * t2710;
    let t2712 = F::cast_from(0.021687161765563047_f64) * t2711;
    let t2715 = F::new(1.0) / t14 / t25 / F::new(4.0);
    let t2716 = t2715 * t2;
    let t2717 = t2716 * t39;
    let t2719 = t732 * t717;
    (t2705, t2706, t2707, t2708, t2709, t2710, t2711, t2712, t2715, t2716, t2717, t2719)
}
