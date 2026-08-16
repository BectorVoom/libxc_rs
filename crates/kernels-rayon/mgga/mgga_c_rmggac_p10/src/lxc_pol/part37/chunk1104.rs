//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1104/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1104(t76228: f64, t76232: f64, t321: f64, t333: f64, t4669: f64, t5155: f64, t69183: f64, t77930: f64, t77933: f64, t77935: f64, t77938: f64, t77940: f64, t77942: f64, t77943: f64, t77946: f64, t77949: f64, t80429: f64) -> f64 {
    let t80434 = 0.82834157616596963771e-1_f64 * t76228;
    let t80435 = 0.16566831523319392754e-1_f64 * t76232;
    let t80442 = -t69183 + t77930 + t77933 - t77935 + t77938 - t77940 - t77942 + t77943 + t77946 - t80434 - t80435 - 0.17961362552795712846e0_f64 * t4669 * t80429 * t321 + 0.23948483403727617128e0_f64 * t5155 * t80429 * t333 - t77949;
    t80442
}
