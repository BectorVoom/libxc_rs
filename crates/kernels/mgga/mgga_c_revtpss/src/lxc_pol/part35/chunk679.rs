//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 679/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk679<F: Float>(t6996: F, t2061: F, t212: F, t780: F, t689: F, t2062: F, t786: F, t789: F, t7023: F, t7031: F, t7034: F, t7041: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7370 = F::new(2.0) / F::new(3.0) * t6996;
    let t7384 = t212 * t2061;
    let t7385 = t7384 * t780;
    let t7387 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t7385;
    let t7388 = t786 * t2062;
    let t7390 = F::cast_from(0.9757440539382783019e-2_f64) * t7388 * t789;
    let t7391 = F::new(7.0) / F::new(144.0) * t7023;
    let t7393 = F::cast_from(0.28582678745379824648e-4_f64) * t7031;
    let t7394 = F::cast_from(0.50820002809285328225e-4_f64) * t7034;
    let t7396 = F::cast_from(0.40015750243531754507e-2_f64) * t7041;
    (t7370, t7384, t7385, t7387, t7388, t7390, t7391, t7393, t7394, t7396)
}
