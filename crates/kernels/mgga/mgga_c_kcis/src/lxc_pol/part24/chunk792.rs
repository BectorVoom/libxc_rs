//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 792/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk792<F: Float>(t330: F, t6338: F, t829: F, t3269: F, t6316: F, t10314: F, t10292: F, t6326: F, t934: F, t10297: F, t1045: F, t18653: F, t4565: F, t14347: F, t18648: F, t18657: F) -> (F, F, F, F, F, F, F) {
    let t18692 = t6338 * t330;
    let t18693 = t18692 * t829;
    let t18694 = t3269 * t18693;
    let t18697 = t6316 * t330;
    let t18698 = t18697 * t829;
    let t18699 = t10314 * t18698;
    let t18703 = t10292 * t6326 * t934;
    let t18707 = t10297 * t6326 * t1045;
    let t18710 = t4565 * t18653;
    let t18713 = t14347 * t18648;
    let t18716 = t4565 * t18657;
    (t18694, t18699, t18703, t18707, t18710, t18713, t18716)
}
