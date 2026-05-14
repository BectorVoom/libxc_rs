//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1105/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1105<F: Float>(t1386: F, t5732: F, t3801: F, t5709: F, t5885: F, t28510: F, t4142: F, t1889: F, t94228: F, t94229: F, t3717: F, t52460: F, t27357: F, t5426: F, t12185: F, t1307: F, t1650: F, t27359: F, t27369: F, t27459: F, t28439: F, t3984: F, t4001: F, t7908: F, t7909: F, t94227: F, t94287: F, t94289: F, t94626: F, t98155: F) -> (F, F, F, F, F) {
    let t98205 = t1386 * t5732;
    let t98220 = t5709 * t5885 * t3801;
    let t98225 = t4142 * t28510;
    let t98226 = 0.14739506172839506172e-2 * t98225;
    let t98230 = t94228 * t1889 * t94229;
    let t98233 = t52460 * t3717;
    let t98235 = t98233 * t5426 * t27357;
    let t98238 = 0.46336805555555555556e-3 * t7908 * t3984 * t98205 * t1307 - 0.16489724537037037037e-3 * t98155 * t27359 - 0.46336805555555555556e-3 * t7908 * t12185 * t7909 * t1650 * t4001 + 0.46336805555555555556e-3 * t27459 * t28439 - 0.46336805555555555556e-3 * t7908 * t98220 - 0.61836467013888888888e-4 * t27369 * t98220 + t98226 + 0.10297067901234567901e-3 * t94287 - 0.15445601851851851852e-3 * t94289 - 0.46336805555555555556e-3 * t94626 * t98230 + 0.82448622685185185186e-4 * t94227 * t98235;
    (t98205, t98225, t98230, t98235, t98238)
}
