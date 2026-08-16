//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1278/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1278(t1615: f64, t30424: f64, t6176: f64, t7429: f64, t28714: f64, t28741: f64, t1394: f64, t5644: f64, t98409: f64, t28356: f64, t5649: f64, t5655: f64) -> (f64, f64, f64, f64, f64) {
    let t101910 = t6176 * t30424 * t7429 * t1615;
    let t101919 = t28714 * t28741;
    let t101922 = t1394 * t98409 * t5644;
    let t101925 = t1394 * t28356 * t5649;
    let t101928 = t1394 * t28356 * t5655;
    (t101910, t101919, t101922, t101925, t101928)
}
