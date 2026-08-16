//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 870/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk870(t1525: f64, t1971: f64, t209: f64, t236: f64, t605: f64, t7453: f64, t1970: f64, t498: f64, t6182: f64, t7231: f64, t321: f64, t3352: f64) -> (f64, f64, f64) {
    let t44627 = t7453 * t1971 * t236 * t1525 * t605 * t209;
    let t44632 = t1970 * t7231 * t236 * t6182 * t498;
    let t44637 = t1970 * t3352 * t236 * t6182 * t321;
    (t44627, t44632, t44637)
}
