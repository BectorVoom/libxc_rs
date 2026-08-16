//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 828/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk828(t18555: f64, t4642: f64, t3255: f64, t6594: f64, t3269: f64, t6330: f64, t934: f64, t1045: f64, t3274: f64, t6326: f64, t829: f64) -> (f64, f64, f64, f64, f64) {
    let t18556 = t4642 * t18555;
    let t18559 = t3255 * t6594;
    let t18563 = t3269 * t6330 * t934;
    let t18567 = t3274 * t6330 * t1045;
    let t18570 = t6326 * t829;
    (t18556, t18559, t18563, t18567, t18570)
}
