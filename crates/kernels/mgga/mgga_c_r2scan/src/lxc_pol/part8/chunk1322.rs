//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1322/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1322<F: Float>(t10059: F, t2294: F, t6132: F, t10025: F, t2598: F, t19807: F, t19845: F, t19978: F, t2122: F, t2124: F, t22749: F, t25243: F, t2591: F, t2612: F, t26141: F, t2634: F, t2654: F, t27644: F, t27650: F, t28390: F, t3071: F, t32309: F, t32314: F, t32319: F, t360: F, t495: F, t5109: F, t7338: F, t9482: F, t9485: F) -> (F,) {
    let t32325 = t6132 * t2294 * t10059;
    let t32328 = t2598 * t2294 * t10025;
    let t32330 = -0.52009330440325611378e0 * t6132 * t5109 * t7338 * t28390 + 0.7801399566048841707e0 * t19807 * t5109 * t3071 * t2654 + 0.52009330440325611378e0 * t19845 * t5109 * t3071 * t2634 + 0.78013995660488417067e0 * t19807 * t5109 * t3071 * t2612 - 0.7801399566048841707e0 * t25243 * t9482 - 0.31205598264195366828e1 * t26141 * t9485 - 0.20803732176130244552e1 * t27644 - 0.38415120233790484326e0 * t27650 + 0.65854491829355115988e0 * t22749 * t2124 * t32309 * t495 + 0.32927245914677557994e0 * t2122 * t2124 * t32314 * t19978 + 0.60677552180379879941e0 * t2598 * t360 * t32319 * t2591 + 0.69345773920434148504e0 * t32325 - 0.69345773920434148504e0 * t32328;
    (t32330,)
}
