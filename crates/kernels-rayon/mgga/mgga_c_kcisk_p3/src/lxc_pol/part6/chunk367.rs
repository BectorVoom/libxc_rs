//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 367/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk367(t1611: f64, t2241: f64, t2283: f64, t2339: f64, t2347: f64, t240: f64, t555: f64, t650: f64, sigma2: f64) -> (f64, f64) {
    let t2351 = t2241 - t2283 + t240 * (-t1611 * t2347 + t2339 * t555 - t2241 + t2283);
    let t2355 = 1.0_f64 / t650;
    let t2356 = sigma2 * t2355;
    (t2351, t2356)
}
