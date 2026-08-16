//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1949/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1949(t16308: f64, t22833: f64, t16123: f64, t2002: f64, t559: f64, t1307: f64, t1377: f64, t22633: f64, t22635: f64, t5353: f64, t26215: f64, t80650: f64) -> (f64, f64, f64, f64) {
    let t91413 = t22833 * t16308;
    let t91416 = t16123 * t2002 * t559;
    let t91449 = t22633 * t22635 * t1377 * t5353 * t1307;
    let t91455 = t22633 * t80650 * t26215;
    (t91413, t91416, t91449, t91455)
}
