//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 830/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk830<F: Float>(t11529: F, t447: F, t445: F, t3845: F, t429: F, t431: F, t1049: F, t442: F, t13964: F, t12951: F, t167: F, t3532: F, t967: F, t143: F, t1390: F, t213: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14057 = t11529 * t447;
    let t14059 = 0.72818958333333333333e-4 * t445 * t14057;
    let t14062 = 0.27323333333333333333e-1 * t429 * t3845 * t431;
    let t14082 = t1049 * t442;
    let t14083 = 0.62154466893555682512e-3 * t14082;
    let t14084 = 0.71734315950379065738e-1 * t13964;
    let t14085 = t167 * t12951;
    let t14090 = t967 * t3532;
    let t14093 = t143 * t3532;
    let t14100 = t213 * t1390;
    (t14059, t14062, t14082, t14083, t14084, t14085, t14090, t14093, t14100)
}
