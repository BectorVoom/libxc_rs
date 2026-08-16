//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1512/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1512(t1445: f64, t3895: f64, t2439: f64, t1420: f64, t2453: f64, t3908: f64, t4067: f64, t786: f64, t1364: f64, t213: f64, t4066: f64, t1426: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10162 = t3895 * t1445;
    let t10163 = t2439 * t10162;
    let t10165 = t2453 * t1420;
    let t10166 = t10165 * t3908;
    let t10168 = t786 * t4067;
    let t10169 = t10168 * t1364;
    let t10171 = t213 * t4066;
    let t10174 = t1420 * t1426;
    (t10162, t10163, t10165, t10166, t10168, t10169, t10171, t10174)
}
