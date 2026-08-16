//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1071/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1071(t26190: f64, t6888: f64, t1834: f64, t214: f64, t6891: f64, t22674: f64, t7691: f64, t22892: f64, t6883: f64, t7701: f64, t5353: f64, t6906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26191 = t6888 * t26190;
    let t26193 = t214 * t1834;
    let t26194 = t26193 * t6891;
    let t26195 = t6888 * t26194;
    let t26197 = t22674 * t7691;
    let t26198 = t22892 * t26197;
    let t26200 = t6883 * t7701;
    let t26202 = t6906 * t5353;
    (t26191, t26193, t26195, t26198, t26200, t26202)
}
