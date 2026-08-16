//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1027/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1027(t14283: f64, t14286: f64, t14289: f64, t14291: f64, t14297: f64, t14300: f64, t14601: f64, t15169: f64, t240: f64, t297: f64, t294: f64, t3132: f64, t3175: f64) -> (f64, f64) {
    let t15171 = t15169 * t240 + t14283 - t14286 + t14289 - t14291 - t14297 + t14300 - t14601;
    let t15172 = t297 * t15171;
    let t15173 = t294 * t15172;
    let t15174 = t15173 / 16.0_f64;
    let t15175 = t3132 * t3175;
    (t15174, t15175)
}
