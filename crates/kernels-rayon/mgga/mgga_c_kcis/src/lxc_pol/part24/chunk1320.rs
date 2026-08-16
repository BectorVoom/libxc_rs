//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1320/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1320(t101761: f64, t29222: f64, t29231: f64, t29234: f64, t29237: f64, t8: f64, t91781: f64, t91785: f64, t91786: f64, t93848: f64, t93849: f64, t93852: f64, t95278: f64, t95279: f64, t95280: f64, t95281: f64, t97606: f64, t97607: f64, t97608: f64, t99810: f64, t99825: f64, t99835: f64) -> f64 {
    let t101765 = -t91781 - t29231 - t91785 - t95278 - t95279 + t91786 - t95280 - t95281 - t97606 + t97607 + t8 * (t99810 + t99825 + t99835 + t101761) + t93848 - t93849 - t97608 - t29234 - t29222 - t29237 + t93852;
    t101765
}
