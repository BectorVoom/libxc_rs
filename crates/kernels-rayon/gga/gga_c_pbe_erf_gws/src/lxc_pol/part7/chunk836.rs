//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 836/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk836(t184: f64, t7631: f64, t197: f64, t5283: f64, t1802: f64, t1885: f64, t1639: f64, t649: f64, t1642: f64, t1: f64, t837: f64, t562: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7632 = t7631 * t184;
    let t7669 = t5283 * t197;
    let t7703 = t1885 * t1802;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7776 = t1 * t837;
    let t7838 = t562 * t577;
    (t7632, t7669, t7703, t7758, t7759, t7776, t7838)
}
