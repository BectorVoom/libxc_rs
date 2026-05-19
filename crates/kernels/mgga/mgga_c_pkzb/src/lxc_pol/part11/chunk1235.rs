//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1235/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1235<F: Float>(t30313: F, t30358: F, t664: F, t684: F, t10868: F, t17536: F, t10892: F, t5771: F, t10777: F, t663: F, t685: F, t17349: F, t17351: F, t20705: F, t20845: F, t25633: F, t25636: F, t261: F, t30284: F, t30287: F) -> (F, F, F, F, F) {
    let t30362 = F::new(1.0) * t664 * (t30313 + t30358) * t684;
    let t30364 = F::cast_from(0.51726012919273400301e3_f64) * t17536 * t10868;
    let t30366 = F::new(6.0) * t5771 * t10892;
    let t30367 = t10777 * t663;
    let t30369 = F::new(1.0) * t30367 * t685;
    let t30377 = (t17349 - F::cast_from(0.28842592592592592592e-1_f64) * t17351 - F::cast_from(0.86527777777777777779e-1_f64) * t20705 + t20845 + F::cast_from(0.37083333333333333333e-1_f64) * t25633 - F::new(0.278125e-1) * t25636 - F::cast_from(0.92708333333333333333e-2_f64) * t30284 + F::new(0.278125e-1) * t30287) * t261;
    (t30362, t30364, t30366, t30369, t30377)
}
