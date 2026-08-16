//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1081/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1081(t30: f64, t41154: f64, t1568: f64, t7063: f64, t33: f64, t116: f64, t29421: f64, t1518: f64, t1936: f64, t670: f64, t7724: f64, t8151: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98785 = t41154 * t30;
    let t98848 = t7063 * t1568;
    let t100981 = t41154 * t33;
    let t104115 = t29421 * t116;
    let t105823 = t1518 * t1936;
    let t108120 = t7724 * t670;
    let t111734 = t8151 * t670;
    (t98785, t98848, t100981, t104115, t105823, t108120, t111734)
}
