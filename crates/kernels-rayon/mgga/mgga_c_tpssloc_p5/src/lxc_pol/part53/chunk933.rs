//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 933/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk933(t1834: f64, t213: f64, t225: f64, t214: f64, t5318: f64, t111: f64, t26966: f64, t26722: f64, t26708: f64, t1509: f64, t7084: f64, t2047: f64, t4233: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t90566 = t213 * t1834 * t225;
    let t90739 = t214 * t5318;
    let t92090 = t26966 * t111;
    let t92386 = t26722 * t225;
    let t92439 = t26708 * t225;
    let t92552 = t7084 * t1509;
    let t92745 = t2047 * t4233;
    (t90566, t90739, t92090, t92386, t92439, t92552, t92745)
}
