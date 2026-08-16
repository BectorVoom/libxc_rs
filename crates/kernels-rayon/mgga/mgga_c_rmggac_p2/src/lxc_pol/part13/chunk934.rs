//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 934/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk934(t25441: f64, t8545: f64, t1970: f64, t236: f64, t321: f64, t3352: f64, t5605: f64, t3351: f64, t511: f64, t5218: f64, t1971: f64, t5184: f64, t880: f64) -> (f64, f64, f64, f64) {
    let t40518 = t25441 * t8545;
    let t40529 = t1970 * t3352 * t236 * t5605 * t321;
    let t40533 = t3351 * t3352 * t511 * t5218;
    let t40537 = t3351 * t1971 * t880 * t5184;
    (t40518, t40529, t40533, t40537)
}
