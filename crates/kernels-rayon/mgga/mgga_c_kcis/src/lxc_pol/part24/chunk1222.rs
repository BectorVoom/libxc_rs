//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1222/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1222(t99904: f64, t99906: f64, t99908: f64, t99910: f64, t99912: f64, t99914: f64, t99917: f64, t99919: f64, t99921: f64, t99923: f64, t99925: f64, t99927: f64, t99929: f64, t99931: f64, t99933: f64, t99935: f64, t99937: f64, t99939: f64, t99941: f64) -> f64 {
    let t99943 = t99904 / 432.0_f64 + t99906 / 64.0_f64 - t99908 / 8.0_f64 - t99910 / 24.0_f64 + t99912 / 12.0_f64 + t99914 / 4.0_f64 + t99917 / 24.0_f64 - t99919 / 288.0_f64 + t99921 / 48.0_f64 + t99923 / 96.0_f64 + t99925 / 128.0_f64 - t99927 / 96.0_f64 + t99929 / 8.0_f64 + t99931 / 18.0_f64 - t99933 / 72.0_f64 + 2.0_f64 / 9.0_f64 * t99935 + t99937 / 64.0_f64 - t99939 / 64.0_f64 - t99941 / 24.0_f64;
    t99943
}
