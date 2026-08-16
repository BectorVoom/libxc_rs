//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1331/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1331(t17311: f64, t28573: f64, t4189: f64, t6048: f64, t8207: f64, t12338: f64, t29430: f64, t1628: f64, t29624: f64, t2069: f64, t28644: f64, t2253: f64, t22714: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102813 = 4.0_f64 * t17311 * t28573;
    let t102816 = 4.0_f64 * t4189 * t8207 * t6048;
    let t102820 = 2.0_f64 * t12338 * t29430;
    let t102823 = t29624 * t1628;
    let t102828 = 4.0_f64 * t4189 * t28644 * t2069;
    let t102833 = 2.0_f64 * t4189 * t2253 * t22714;
    (t102813, t102816, t102820, t102823, t102828, t102833)
}
