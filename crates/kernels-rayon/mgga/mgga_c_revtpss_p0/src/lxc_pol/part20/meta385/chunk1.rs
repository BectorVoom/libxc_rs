//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1415/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1415(t11509: f64, t41224: f64, t41225: f64, t981: f64, t3010: f64, t3013: f64, t11616: f64, t3022: f64, t241: f64, t281: f64, t283: f64, t11144: f64, t2251: f64, t2258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41229 = 0.12304822629859687989e5_f64 * t981 * t41224 * t41225 * t11509;
    let t41234 = t3010 * t3010;
    let t41235 = 1.0_f64 / t41234;
    let t41237 = t3013 * t3013;
    let t41238 = 1.0_f64 / t41237;
    let t41241 = 0.91082604192152556044e5_f64 * t981 * t41235 * t41225 * t41238;
    let t41243 = 0.4101607543286562663e4_f64 * t3022 * t11616;
    let t41245 = t281 * t241 * t283;
    let t41246 = 0.13490888888888888889e1_f64 * t41245;
    let t41248 = t11144 * t2251 * t2258;
    (t41229, t41235, t41238, t41241, t41243, t41245, t41246, t41248)
}
