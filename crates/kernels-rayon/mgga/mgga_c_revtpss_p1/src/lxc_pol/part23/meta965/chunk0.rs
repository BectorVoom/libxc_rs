//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3262/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3262(t73352: f64, t177: f64, t22789: f64, t762: f64, t48227: f64, t46973: f64, t48243: f64, t46977: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t48224: f64, t48226: f64, t48234: f64, t48236: f64, t48241: f64, t48244: f64, t48248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t85893 = 0.17544670867903938621e1_f64 * t73352;
    let t85895 = t22789 * t177 * t762;
    let t85896 = 0.5848223622634646207e0_f64 * t85895;
    let t85897 = 180.0_f64 * t48227;
    let t85898 = 12.0_f64 * t46973;
    let t85899 = 3.0_f64 * t48243;
    let t85900 = 120.0_f64 * t46977;
    let t85901 = -t85893 - t48224 - t39483 - t48226 + t39520 - t85896 + t85897 - t39528 - t85898 + t39531 + t48234 + t48236 + t48241 + t85899 - t48244 - t85900 + t48248;
    (t85893, t85896, t85897, t85898, t85899, t85900, t85901)
}
