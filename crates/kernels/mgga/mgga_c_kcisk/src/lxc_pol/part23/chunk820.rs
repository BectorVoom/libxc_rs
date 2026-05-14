//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 820/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk820<F: Float>(t15: F, t944: F, t1014: F, t142: F, t3088: F, t5: F, t119: F, t955: F, t1049: F, t213: F, t5816: F, t5823: F, t5827: F, t181: F, t3086: F, t21: F, t3117: F, t3201: F) -> (F, F, F, F, F, F, F) {
    let t12407 = t15 * t944;
    let t12408 = t1014 * t12407;
    let t12410 = t5 * t142 * t3088;
    let t12414 = t5 * t119 * t955;
    let t12425 = 0.35867157975189532869e-1 * t213 - 0.13661666666666666667e-1 * t5827 + 0.38744444444444444446e-2 * t5816 - 0.15538616723388920628e-3 * t1049 + 0.18204739583333333333e-4 * t5823;
    let t12434 = t181 * t3086;
    let t12435 = t3088 * t955;
    let t12436 = t12434 * t12435;
    let t12442 = t3201 * t21 * t3117;
    (t12408, t12410, t12414, t12425, t12435, t12436, t12442)
}
