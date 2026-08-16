//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1400/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1400(t6864: f64, t9918: f64, t1353: f64, t6816: f64, t4012: f64, t828: f64, t3930: f64, t6876: f64, t1883: f64, t5627: f64, t13783: f64, t13926: f64, t6869: f64) -> (f64, f64, f64, f64, f64) {
    let t22285 = t9918 * t6864;
    let t22287 = t6816 * t1353;
    let t22289 = t4012 * t828 * t22287;
    let t22292 = t3930 * t6876;
    let t22294 = t1883 * t5627;
    let t22295 = t13783 * t22294;
    let t22298 = t13926 * t6869;
    (t22285, t22289, t22292, t22295, t22298)
}
