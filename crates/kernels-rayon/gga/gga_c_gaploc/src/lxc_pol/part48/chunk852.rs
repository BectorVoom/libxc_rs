//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 852/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk852(t2268: f64, t35901: f64, t894: f64, t426: f64, t44386: f64, t535: f64, t13258: f64, t484: f64, t11481: f64, t2321: f64, t882: f64, t1063: f64, t11271: f64, t6750: f64) -> (f64, f64, f64, f64, f64) {
    let t44618 = 0.56910013271352299198e-1_f64 * t2268 * t894 * t35901;
    let t44622 = 0.28455006635676149599e-1_f64 * t2268 * t535 * t44386 * t426;
    let t44623 = t484 * t13258;
    let t44624 = 0.15808337019820083111e-2_f64 * t44623;
    let t44626 = t882 * t11481 * t2321;
    let t44627 = 0.11856252764865062333e-2_f64 * t44626;
    let t44630 = 0.85365019907028448797e-1_f64 * t1063 * t11271 * t6750;
    (t44618, t44622, t44624, t44627, t44630)
}
