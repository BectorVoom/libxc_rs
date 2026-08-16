//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 897/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk897<F: Float>(t1936: F, t670: F, t1518: F, t572: F, t26123: F, t4292: F, t7330: F, t1459: F, t7953: F, t116: F, t7741: F, t117: F, t28042: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28264 = t670 * t1936;
    let t28265 = t28264 * t1518;
    let t28267 = F::cast_from(6.0_f64) * t572 * t28265;
    let t28268 = t26123 * t1518;
    let t28270 = F::cast_from(6.0_f64) * t572 * t28268;
    let t28271 = t7330 * t4292;
    let t28273 = F::cast_from(6.0_f64) * t572 * t28271;
    let t28275 = F::cast_from(3.0_f64) * t1459 * t7953;
    let t28276 = t116 * t7741;
    let t28277 = t28276 * t670;
    let t28279 = F::cast_from(6.0_f64) * t572 * t28277;
    let t28280 = t117 * t28042;
    (t28264, t28265, t28267, t28268, t28270, t28271, t28273, t28275, t28277, t28279, t28280)
}
