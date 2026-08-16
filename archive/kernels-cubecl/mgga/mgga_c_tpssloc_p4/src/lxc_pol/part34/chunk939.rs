//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 939/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk939<F: Float>(t1100: F, t21780: F, t1661: F, t5992: F, t11265: F, t21762: F, t3297: F, t136: F, t1113: F, t21769: F, t21776: F, t11219: F, t21758: F) -> (F, F, F, F, F, F, F) {
    let t21783 = t1100 * t21780;
    let t21785 = t5992 * t1661;
    let t21786 = t11265 * t21785;
    let t21788 = t3297 * t21762;
    let t21789 = t136 * t21788;
    let t21791 = t1113 * t21769;
    let t21792 = t136 * t21791;
    let t21794 = t1113 * t21776;
    let t21795 = t136 * t21794;
    let t21801 = t11219 * t21758;
    (t21783, t21785, t21786, t21789, t21792, t21795, t21801)
}
