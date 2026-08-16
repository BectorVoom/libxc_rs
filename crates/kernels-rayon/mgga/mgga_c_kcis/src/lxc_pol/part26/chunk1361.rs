//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1361/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1361(t1385: f64, t1889: f64, t1938: f64, t94228: f64, t833: f64, t27370: f64, t5732: f64, t5885: f64, t28335: f64, t28369: f64, t102085: f64, t102102: f64, t102106: f64, t102109: f64, t103191: f64, t27369: f64, t28348: f64, t52697: f64, t5440: f64, t94626: f64, t98155: f64, t98255: f64, t98270: f64, t98653: f64) -> (f64, f64, f64, f64) {
    let t103328 = t94228 * t1889 * t1938 * t1385;
    let t103331 = t1938 * t833;
    let t103340 = t27370 * t5885 * t5732;
    let t103343 = t28369 * t28335;
    let t103347 = t98255 + 0.4946917361111111111e-3_f64 * t98155 * t28348 + 0.3684876543209876543e-2_f64 * t102085 - 0.92673611111111111112e-3_f64 * t94626 * t98653 * t1889 * t52697 - 0.46336805555555555556e-3_f64 * t94626 * t103328 - 0.92673611111111111112e-3_f64 * t94626 * t98270 * t5440 * t103331 - 0.55273148148148148147e-3_f64 * t102102 - 0.18550940104166666667e-3_f64 * t27369 * t103191 - 0.18550940104166666667e-3_f64 * t27369 * t103340 - 0.15445601851851851852e-3_f64 * t103343 + 0.99491666666666666664e-2_f64 * t102106 - 0.33163888888888888888e-2_f64 * t102109;
    (t103328, t103331, t103340, t103347)
}
