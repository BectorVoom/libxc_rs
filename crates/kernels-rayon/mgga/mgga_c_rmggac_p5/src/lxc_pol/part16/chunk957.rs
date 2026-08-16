//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 957/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk957(t39490: f64, t8571: f64, t2144: f64, t3351: f64, t498: f64, t6557: f64, t7231: f64, t3352: f64, t6583: f64, t1971: f64, t6586: f64, t7190: f64) -> (f64, f64, f64, f64) {
    let t45951 = t8571 * t39490;
    let t45956 = t3351 * t7231 * t2144 * t6557 * t498;
    let t45960 = t3351 * t3352 * t2144 * t6583;
    let t45964 = t3351 * t1971 * t7190 * t6586;
    (t45951, t45956, t45960, t45964)
}
