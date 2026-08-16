//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 747/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk747<F: Float>(t70271: F, t14530: F, t290: F, t14580: F, t899: F, t70316: F, t70328: F, t70376: F, t70385: F, t70439: F, t2228: F, t265: F) -> (F, F, F, F, F, F, F, F, F) {
    let t71755 = F::cast_from(0.6505345598561924296e-5_f64) * t70271;
    let t71760 = t290 * t14530;
    let t71772 = t899 * t14580;
    let t71775 = F::cast_from(0.6505345598561924296e-5_f64) * t70316;
    let t71789 = F::cast_from(0.3830813990396805546e-3_f64) * t70328;
    let t71802 = F::cast_from(0.162600798888400151e-2_f64) * t70376;
    let t71804 = F::cast_from(0.32526727992809621482e-4_f64) * t70385;
    let t71832 = F::cast_from(0.2316441583394736328e-4_f64) * t70439;
    let t71835 = t2228 * t265;
    (t71755, t71760, t71772, t71775, t71789, t71802, t71804, t71832, t71835)
}
