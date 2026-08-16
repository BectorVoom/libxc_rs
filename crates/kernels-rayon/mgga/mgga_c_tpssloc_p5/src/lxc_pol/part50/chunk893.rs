//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 893/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk893(t23384: f64, t6707: f64, t6695: f64, t6680: f64, t6683: f64, t6699: f64, t968: f64, t1920: f64, t225: f64, t3173: f64) -> (f64, f64, f64, f64, f64) {
    let t23385 = t23384 * t6707;
    let t23387 = t23384 * t6695;
    let t23389 = t6680 * t6683;
    let t23391 = t968 * t6699;
    let t23392 = t1920 * t23391;
    let t23394 = t225 * t3173;
    (t23385, t23387, t23389, t23392, t23394)
}
