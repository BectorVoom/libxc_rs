//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 898/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk898(t45043: f64, t7474: f64, t1970: f64, t1971: f64, t236: f64, t6178: f64, t1704: f64, t209: f64, t476: f64, t9188: f64, t1707: f64, t3352: f64, t495: f64, t511: f64, t7230: f64) -> (f64, f64, f64, f64) {
    let t45044 = t7474 * t45043;
    let t45048 = t1970 * t1971 * t236 * t6178;
    let t45055 = t1970 * t9188 * t236 * t1704 * t476 * t209;
    let t45060 = t7230 * t3352 * t511 * t1707 * t495;
    (t45044, t45048, t45055, t45060)
}
