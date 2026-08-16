//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1440/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1440(t3869: f64, t9575: f64, t1331: f64, t3860: f64, t1320: f64, t3855: f64, t186: f64, t685: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9577 = 0.21687162600603479684e-1_f64 * t3869 * t9575;
    let t9578 = t3860 * t1331;
    let t9579 = 36.0_f64 * t9578;
    let t9580 = t1320 * t3855;
    let t9581 = 12.0_f64 * t9580;
    let t9586 = t685 * t793 * t186;
    (t9577, t9578, t9579, t9580, t9581, t9586)
}
