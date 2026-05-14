//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1359/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1359<F: Float>(t123: F, t2801: F, t34578: F, t34415: F, t9720: F, t2647: F, t33197: F, t5515: F, t7261: F, t33234: F, t9991: F, t33162: F, t34484: F, t9721: F, t34594: F, t10000: F, t10005: F, t116293: F, t2807: F, t33173: F, t33188: F, t33193: F, t33196: F, t33204: F, t34416: F, t34579: F, t9743: F) -> (F, F) {
    let t117784 = t2801 * t34578 * t123;
    let t117791 = t9720 * t34415;
    let t117796 = t7261 * t33197 * t2647 * t5515;
    let t117808 = 0.34722222222222222222e-2 * t9991 * t33234;
    let t117810 = 0.34722222222222222222e-2 * t9991 * t33162;
    let t117812 = 0.34722222222222222222e-2 * t9721 * t34484;
    let t117814 = 0.13402777777777777778e-2 * t34594 * t33162;
    let t117815 = 0.92592592592592592594e-2 * t117784 * t9743 - 0.17361111111111111111e-2 * t34416 * t33173 - 0.23148148148148148148e-2 * t34416 * t33204 - 0.34722222222222222222e-2 * t117791 * t9743 - 0.20104166666666666667e-2 * t33196 * t117796 + 0.27777777777777777778e-1 * t9720 * t34579 * t2807 + 0.34822083333333333332e-2 * t116293 - 0.13888888888888888889e-1 * t10005 * t33188 + 0.52083333333333333333e-2 * t10000 * t33193 + t117808 + t117810 + t117812 + t117814;
    (t117796, t117815)
}
