//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1099/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1099(t26946: f64, t7754: f64, t26918: f64, t26920: f64, t26922: f64, t26925: f64, t26927: f64, t26931: f64, t26934: f64, t26936: f64, t26939: f64, t26942: f64, t26944: f64) -> (f64, f64) {
    let t26947 = t7754 * t26946;
    let t26949 = -t26918 / 16.0_f64 + t26920 / 16.0_f64 + 11.0_f64 / 18.0_f64 * t26922 - 2.0_f64 / 9.0_f64 * t26925 - t26927 / 12.0_f64 + t26931 / 48.0_f64 - t26934 / 8.0_f64 - t26936 / 3.0_f64 + t26939 / 12.0_f64 + t26942 / 24.0_f64 - t26944 / 128.0_f64 - t26947 / 72.0_f64;
    (t26947, t26949)
}
