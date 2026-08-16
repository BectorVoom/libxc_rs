//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1170/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1170(t11749: f64, t33643: f64, t933: f64, t11790: f64, t3367: f64, t6188: f64, t11794: f64, t7927: f64, t9554: f64, t126: f64, t671: f64, t128: f64, t314: f64) -> (f64, f64, f64, f64, f64) {
    let t33645 = t933 * t33643 * t11749;
    let t33648 = t11790 * t3367 * t6188;
    let t33653 = t11794 * t7927 * t9554;
    let t33655 = t126 * t671;
    let t33657 = t314 * t128;
    (t33645, t33648, t33653, t33655, t33657)
}
