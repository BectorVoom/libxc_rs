//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 588/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk588<F: Float>(t7692: F, t1240: F, t128: F, t118: F, t1986: F, t1994: F, t1249: F, t687: F, t4685: F, t681: F, t4616: F, t664: F) -> (F, F, F, F, F, F) {
    let t7693 = F::cast_from(0.1064114997332445985e-4_f64) * t7692;
    let t7694 = t128 * t1240;
    let t7695 = t118 * t7694;
    let t7696 = t1986 * t7695;
    let t7697 = t1994 * t7696;
    let t7698 = F::cast_from(0.53205749866622299248e-5_f64) * t7697;
    let t7699 = t1249 * t687;
    let t7700 = F::cast_from(0.19957069503106347607e-1_f64) * t7699;
    let t7701 = t4685 * t681;
    let t7702 = F::cast_from(0.14967802127329760705e-1_f64) * t7701;
    let t7703 = t4616 * t664;
    (t7693, t7696, t7698, t7700, t7702, t7703)
}
