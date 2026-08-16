//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 808/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk808(t2808: f64, t511: f64, t3352: f64, t14258: f64, t2841: f64, t495: f64, t14230: f64, t14249: f64, t2067: f64, t14124: f64, t14125: f64, t201: f64, t457: f64, t558: f64) -> (f64, f64, f64) {
    let t74555 = t511 * t2808;
    let t74556 = t3352 * t74555;
    let t74557 = t14258 * t74556;
    let t74559 = t2841 * t495;
    let t74562 = t14230 * t14249 * t2067 * t74559;
    let t74569 = t14124 * t14125 * t511 * t558 * t457 * t201;
    (t74557, t74562, t74569)
}
