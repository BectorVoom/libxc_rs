//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1063/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1063(t1907: f64, t236: f64, t498: f64, t7230: f64, t7231: f64, t1652: f64, t1971: f64, t515: f64, t615: f64, t2144: f64, t495: f64, t6557: f64) -> (f64, f64, f64) {
    let t47455 = t7230 * t7231 * t236 * t1907 * t498;
    let t47460 = t7230 * t1971 * t515 * t1652 * t615;
    let t47465 = t7230 * t1971 * t2144 * t6557 * t495;
    (t47455, t47460, t47465)
}
