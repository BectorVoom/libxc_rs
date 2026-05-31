//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1946/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1946<F: Float>(t1916: F, t7331: F, t7334: F, t1459: F, t7950: F, t1936: F, t670: F, t1518: F, t572: F, t26123: F, t4292: F, t7330: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28259 = F::cast_from(6.0_f64) * t1916 * t7331;
    let t28261 = F::cast_from(3.0_f64) * t1916 * t7334;
    let t28263 = F::cast_from(6.0_f64) * t1459 * t7950;
    let t28264 = t670 * t1936;
    let t28265 = t28264 * t1518;
    let t28267 = F::cast_from(6.0_f64) * t572 * t28265;
    let t28268 = t26123 * t1518;
    let t28270 = F::cast_from(6.0_f64) * t572 * t28268;
    let t28271 = t7330 * t4292;
    (t28259, t28261, t28263, t28264, t28265, t28267, t28268, t28270, t28271)
}
