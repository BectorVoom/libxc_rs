//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 639/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk639(t609: f64, t2104: f64, t4455: f64, t1610: f64, t6176: f64, t6136: f64) -> (f64, f64, f64, f64) {
    let t614 = 0.0_f64 < t609;
    let t6177 = t4455 * t2104;
    let t6178 = t6177 * t1610;
    let t6179 = t6176 * t6178;
    let t6183 = piecewise3(t614, t6136, -t6136);
    (t6177, t6178, t6179, t6183)
}
