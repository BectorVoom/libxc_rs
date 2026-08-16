//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2351/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2351(t94223: f64, t94236: f64, t94257: f64, t94272: f64, t95970: f64, t96228: f64, t96232: f64, t96274: f64, t2174: f64, t5363: f64, t1404: f64, t8110: f64) -> (f64, f64, f64) {
    let t96277 = t94223 + t94236 + t94257 + t94272 + t95970 + t96228 + t96232 + t96274;
    let t96281 = 2.0_f64 * t5363 * t2174;
    let t96283 = 2.0_f64 * t8110 * t1404;
    (t96277, t96281, t96283)
}
