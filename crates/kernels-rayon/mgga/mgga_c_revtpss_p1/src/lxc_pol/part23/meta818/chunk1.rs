//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2666/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2666(t11274: f64, t20029: f64, t11656: f64, t19920: f64, t11262: f64, t3127: f64, t6262: f64, t15817: f64, t4820: f64, t15775: f64, t4834: f64, t1032: f64, t1040: f64, t19856: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65585 = t11274 * t20029;
    let t65589 = t11656 * t19920;
    let t65596 = t3127 * t11262 * t6262;
    let t65598 = t15817 * t4820;
    let t65610 = t4834 * t15775;
    let t65613 = t19856 * t1032 * t1040;
    (t65585, t65589, t65596, t65598, t65610, t65613)
}
