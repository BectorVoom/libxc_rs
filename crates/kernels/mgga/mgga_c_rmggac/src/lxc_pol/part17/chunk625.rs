//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 625/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk625<F: Float>(t1550: F, t9765: F, t2298: F, t5055: F, t1856: F, t194: F, t201: F, t1979: F, t1982: F, t2320: F, t8676: F, t128: F, t1907: F, t118: F, t1986: F, t1994: F) -> (F, F, F, F, F, F, F, F) {
    let t9766 = t1550 * t9765;
    let t9767 = 0.2993560425465952141e-1 * t9766;
    let t9770 = t5055 * t2298;
    let t9771 = 0.8980681276397856423e-1 * t9770;
    let t9774 = t194 * t1856;
    let t9775 = t9774 * t201;
    let t9777 = t9775 * t1979 * t1982;
    let t9778 = 0.42564599893297839398e-5 * t9777;
    let t9779 = t8676 * t2320;
    let t9780 = 0.1064114997332445985e-4 * t9779;
    let t9781 = t128 * t1907;
    let t9782 = t118 * t9781;
    let t9783 = t1986 * t9782;
    let t9784 = t1994 * t9783;
    (t9767, t9771, t9774, t9775, t9778, t9780, t9783, t9784)
}
