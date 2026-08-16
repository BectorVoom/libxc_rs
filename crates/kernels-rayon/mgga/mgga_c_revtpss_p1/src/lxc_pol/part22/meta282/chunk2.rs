//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1694/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1694(t3869: f64, t9575: f64, t1331: f64, t3860: f64, t1320: f64, t3855: f64, t186: f64, t685: f64, t793: f64) -> (f64, f64, f64, f64) {
    let t9577 = 0.21687162600603479684e-1_f64 * t3869 * t9575;
    let t9578 = t3860 * t1331;
    let t9580 = t1320 * t3855;
    let t9586 = t685 * t793 * t186;
    (t9577, t9578, t9580, t9586)
}
