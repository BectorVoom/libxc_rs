//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 795/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk795(t299: f64, t5674: f64, t2054: f64, t2057: f64, t2082: f64, t2091: f64, t276: f64, t5630: f64, t5637: f64, t5641: f64, t5646: f64, t5649: f64, t5658: f64, t5661: f64, t5666: f64, t735: f64, t744: f64, t782: f64) -> (f64, f64) {
    let t5675 = t299 * t5674;
    let t5677 = 0.25724410870841842184e-2_f64 * t5630 - 0.51448821741683684368e-2_f64 * t299 * t5637 - 0.42874018118069736972e-3_f64 * t299 * t5641 - t5646 / 96.0_f64 - t276 * t5649 / 96.0_f64 - 11.0_f64 / 36.0_f64 * t2057 * t744 - t735 * t2091 / 6.0_f64 - t276 * t5658 / 16.0_f64 + t5661 / 18.0_f64 + t5666 / 48.0_f64 + t735 * t2054 / 12.0_f64 - 0.43445671692977333464e-1_f64 * t2082 * t782 + 0.28582678745379824648e-3_f64 * t5675;
    (t5675, t5677)
}
