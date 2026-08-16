//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3001/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3001(t2439: f64, t4469: f64, t780: f64, t785: f64, t213: f64, t252: f64, t2440: f64, t4534: f64, t1580: f64, t41117: f64, t10509: f64, t10995: f64, t14990: f64) -> (f64, f64, f64, f64, f64) {
    let t50236 = t2439 * t785 * t4469 * t780;
    let t50240 = t213 * t252;
    let t50245 = t2439 * t2440 * t4534;
    let t50248 = t41117 * t1580;
    let t50253 = t10995 * t14990 * t10509;
    (t50236, t50240, t50245, t50248, t50253)
}
