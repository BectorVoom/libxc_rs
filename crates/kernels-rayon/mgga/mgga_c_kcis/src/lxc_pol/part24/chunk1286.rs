//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1286/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1286(t100179: f64, t101035: f64, t101043: f64, t101047: f64, t101053: f64, t101057: f64, t26685: f64, t26748: f64, t27775: f64, t27780: f64, t27832: f64, t28984: f64, t28988: f64, t93366: f64, t95686: f64) -> f64 {
    let t101060 = 0.46336805555555555556e-3_f64 * t26748 * t28984 - 0.7722800925925925926e-4_f64 * t101035 - 0.22109259259259259259e-2_f64 * t100179 - 0.27802083333333333334e-2_f64 * t27832 * t27775 - 0.13901041666666666667e-2_f64 * t27832 * t27780 + 0.2782641015625e-3_f64 * t26685 * t101043 + 0.10203017057291666667e-2_f64 * t26685 * t101047 - 0.18550940104166666667e-3_f64 * t93366 * t28988 - 0.556528203125e-3_f64 * t26685 * t101053 - 0.18550940104166666667e-3_f64 * t26685 * t101057 - t95686;
    t101060
}
