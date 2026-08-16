//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2792/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2792(t213: f64, t225: f64, t40321: f64, t10872: f64, t14502: f64, t14546: f64, t14972: f64, t2646: f64, t39612: f64, t39617: f64, t39622: f64, t4494: f64, t4504: f64, t4514: f64, t50666: f64, t50758: f64, t50916: f64, t51299: f64, t51306: f64, t820: f64, t837: f64, t879: f64) -> f64 {
    let t51320 = t213 * t225 * t40321;
    let t51327 = -t51299 - 0.29272321618148349057e-1_f64 * t39612 - 0.9757440539382783019e-2_f64 * t39617 + 0.16463622957338778996e-1_f64 * t39622 - 0.19756347548806534796e1_f64 * t4514 * t14502 * t2646 - 0.19756347548806534796e1_f64 * t4514 * t51306 * t837 + 0.13170898365871023197e1_f64 * t4504 * t4494 * t50666 - 0.65854491829355115987e0_f64 * t820 * t879 * t50916 - 0.19756347548806534796e1_f64 * t820 * t14972 * t2646 + 0.15805078039045227836e2_f64 * t51320 * t4494 * t50758 - 0.23707617058567841754e2_f64 * t14546 * t4494 * t10872;
    t51327
}
