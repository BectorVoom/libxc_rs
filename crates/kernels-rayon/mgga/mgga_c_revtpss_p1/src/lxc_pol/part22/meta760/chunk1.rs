//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2841/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2841(t3075: f64, t3154: f64, t11671: f64, t11865: f64, t11725: f64, t828: f64, t11660: f64, t2258: f64, t3204: f64, t3230: f64, t225: f64, t42059: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43116 = t3154 * t3075;
    let t43121 = t11865 * t11671;
    let t43131 = t828 * t11725;
    let t43139 = t11660 * t2258;
    let t43151 = t3204 * t3230;
    let t43154 = t42059 * t225;
    (t43116, t43121, t43131, t43139, t43151, t43154)
}
