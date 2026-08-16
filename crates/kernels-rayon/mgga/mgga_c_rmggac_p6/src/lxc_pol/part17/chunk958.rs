//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 958/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk958(t10088: f64, t495: f64, t511: f64, t7230: f64, t7231: f64, t1737: f64, t3351: f64, t498: f64, t880: f64, t3352: f64, t6394: f64, t1971: f64, t3924: f64, t6397: f64) -> (f64, f64, f64, f64) {
    let t45896 = t7230 * t7231 * t511 * t10088 * t495;
    let t45901 = t3351 * t7231 * t880 * t1737 * t498;
    let t45905 = t3351 * t3352 * t880 * t6394;
    let t45909 = t3351 * t1971 * t3924 * t6397;
    (t45896, t45901, t45905, t45909)
}
