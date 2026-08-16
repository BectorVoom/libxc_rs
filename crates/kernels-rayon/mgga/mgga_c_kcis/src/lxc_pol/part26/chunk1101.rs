//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1101/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1101(t28426: f64, t7898: f64, t5628: f64, t7931: f64, t303: f64, t1307: f64, t28373: f64, t3984: f64, t5885: f64, t5709: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28427 = t7898 * t28426;
    let t28429 = t7931 * t5628;
    let t28430 = t303 * t28429;
    let t28438 = t28373 * t1307;
    let t28439 = t3984 * t28438;
    let t28442 = t5885 * t1307;
    let t28443 = t5709 * t28442;
    (t28427, t28429, t28430, t28438, t28439, t28442, t28443)
}
