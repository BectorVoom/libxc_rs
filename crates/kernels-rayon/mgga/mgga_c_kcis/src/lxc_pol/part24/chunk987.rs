//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 987/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk987(t20749: f64, t20769: f64, t20789: f64, t20809: f64, t11223: f64, t11230: f64, t1282: f64, t1291: f64, t15109: f64, t15692: f64, t1872: f64, t20709: f64, t20711: f64, t20721: f64, t20724: f64, t20728: f64, t3664: f64, t3669: f64, t437: f64, t5360: f64, t5363: f64, t5394: f64, t6860: f64, t6879: f64) -> (f64, f64) {
    let t20811 = t20749 + t20769 + t20789 + t20809;
    let t20813 = 2.0_f64 * t11223 * t6860 - 6.0_f64 * t11230 * t20721 - t1282 * t20811 - t1291 * t20711 - 2.0_f64 * t15109 * t1872 + 4.0_f64 * t15692 * t5363 + t20709 * t437 + 4.0_f64 * t20724 * t3669 + 2.0_f64 * t20728 * t3669 - t3664 * t6879 - 2.0_f64 * t5360 * t5394;
    (t20811, t20813)
}
