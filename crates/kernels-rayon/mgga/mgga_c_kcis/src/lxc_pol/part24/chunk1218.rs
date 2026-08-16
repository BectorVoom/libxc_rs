//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1218/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1218(t19908: f64, t28024: f64, t26938: f64, t29059: f64, t1196: f64, t18463: f64, t19882: f64, t95391: f64, t20155: f64, t283: f64, t7749: f64, t99865: f64, t99867: f64, t99869: f64, t99871: f64, t99874: f64, t99876: f64, t99878: f64, t99880: f64, t99882: f64, t99884: f64, t99886: f64, t99888: f64, t99890: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99892 = t28024 * t19908;
    let t99894 = t26938 * t29059;
    let t99896 = t18463 * t1196;
    let t99898 = t95391 * t19882;
    let t99900 = t20155 * t283;
    let t99901 = t99900 * t7749;
    let t99903 = t99865 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t99867 + t99869 / 12.0_f64 - t99871 / 64.0_f64 - t99874 / 32.0_f64 + t99876 / 8.0_f64 - t99878 / 128.0_f64 + t99880 / 288.0_f64 - t99882 / 48.0_f64 + t99884 / 96.0_f64 + t99886 / 6.0_f64 + t99888 / 6.0_f64 + t99890 / 144.0_f64 - t99892 / 288.0_f64 - t99894 / 72.0_f64 + t99896 / 24.0_f64 + 3.0_f64 / 64.0_f64 * t99898 - t99901 / 16.0_f64;
    (t99892, t99894, t99896, t99898, t99901, t99903)
}
