//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 795/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk795(t10763: f64, t2843: f64, t840: f64, t2894: f64, t824: f64, t1882: f64, t2803: f64, t8232: f64, t842: f64, t10246: f64, t10259: f64, t10265: f64, t10269: f64, t10273: f64, t10276: f64, t10279: f64, t10282: f64, t10391: f64, t10394: f64, t10400: f64, t10624: f64, t10634: f64) -> (f64, f64, f64, f64, f64) {
    let t10765 = t840 * t2843 * t10763;
    let t10769 = t840 * t2894 * t824;
    let t10771 = t1882 * t2803;
    let t10773 = t8232 * t842;
    let t10786 = -t10391 + t10394 - 4.0_f64 / 3.0_f64 * t10400 - 6.0_f64 * t10265 - 2.0_f64 * t10276 + t10624 / 2.0_f64 + 3.0_f64 / 8.0_f64 * t10634 - 2.0_f64 / 3.0_f64 * t10246 - t10259 / 3.0_f64 + 6.0_f64 * t10269 - 10.0_f64 / 27.0_f64 * t10273 - 4.0_f64 / 9.0_f64 * t10279 + t10282 / 3.0_f64;
    (t10765, t10769, t10771, t10773, t10786)
}
