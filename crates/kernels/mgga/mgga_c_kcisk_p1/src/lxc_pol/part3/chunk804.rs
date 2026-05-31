//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 804/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk804<F: Float>(t12405: F, t15: F, t944: F, t1014: F, t142: F, t3088: F, t5: F, t119: F, t955: F, t1049: F, t213: F, t5816: F, t5823: F, t5827: F) -> (F, F, F, F, F) {
    let t12406 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t12405;
    let t12407 = t15 * t944;
    let t12408 = t1014 * t12407;
    let t12410 = t5 * t142 * t3088;
    let t12414 = t5 * t119 * t955;
    let t12425 = F::cast_from(0.35867157975189532869e-1_f64) * t213 - F::cast_from(0.13661666666666666667e-1_f64) * t5827 + F::cast_from(0.38744444444444444446e-2_f64) * t5816 - F::cast_from(0.15538616723388920628e-3_f64) * t1049 + F::cast_from(0.18204739583333333333e-4_f64) * t5823;
    (t12406, t12408, t12410, t12414, t12425)
}
