//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1180/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1180(t180: f64, t2165: f64, t228: f64, t26425: f64, t26561: f64, t2772: f64, t36429: f64, t36439: f64, t7657: f64, t7669: f64, t9010: f64, t9017: f64, t9018: f64, t91791: f64, t91793: f64, t9185: f64, t91863: f64, t91866: f64, t91869: f64, t91872: f64, t91874: f64, t91902: f64, t91905: f64, t91963: f64, t92019: f64, t92064: f64, t92104: f64, t92158: f64, t92165: f64, t92168: f64, t92170: f64, t92339: f64, t92344: f64, t92376: f64) -> f64 {
    let t92379 = t180 * (t91791 + t91793 + t91863 + 6.0_f64 * t91902 * t2772 - t91866 + t91869 - t91872 + t91874 - 6.0_f64 * t91905 * t9018 + (t91963 + t92019 + t92064 + t92104) * t228 + t92158 - 18.0_f64 * t9017 * t7669 * t2772 - t36429 * t2165 - 18.0_f64 * t36439 * t26425 + 12.0_f64 * t9010 * t26561 - t7657 * t9185 + t92165 - t92168 - t92170 - t92339 - t92344 + t92376);
    t92379
}
