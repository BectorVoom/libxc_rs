//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2771/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2771(t2490: f64, t2494: f64, t2538: f64, t268: f64, t675: f64, t9310: f64, t9314: f64) -> (f64, f64, f64, f64) {
    let t39959 = t2490 * t2490;
    let t39960 = 1.0_f64 / t39959;
    let t39962 = t2494 * t2494;
    let t39963 = 1.0_f64 / t39962;
    let t39967 = t2538 * t2538;
    let t39989 = 0.3684616320282908548e2_f64 * t268 * t675 * t9310 * t9314;
    (t39960, t39963, t39967, t39989)
}
