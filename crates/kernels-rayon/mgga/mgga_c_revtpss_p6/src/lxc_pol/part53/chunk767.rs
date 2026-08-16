//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 767/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk767(t225: f64, t494: f64, t8190: f64, t1769: f64, t2142: f64, t7637: f64, t1774: f64) -> (f64, f64, f64, f64) {
    let t8192 = t8190 * t225 * t494;
    let t8197 = t2142 * t1769;
    let t8198 = t7637 * t8197;
    let t8201 = t2142 * t1774;
    (t8192, t8197, t8198, t8201)
}
