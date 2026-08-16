//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1490/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1490(t2349: f64, t43: f64, t10227: f64, t96: f64, t100: f64, t613: f64, t10199: f64, t2175: f64, t2289: f64, t8264: f64, t31051: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t116942 = t43 * t2349;
    let t116946 = t96 * t10227;
    let t116957 = t613 * t100;
    let t116968 = 154.0_f64 / 27.0_f64 * t10199 * t2175;
    let t116969 = t2289 * t8264;
    let t116971 = t625 * t31051;
    (t116942, t116946, t116957, t116968, t116969, t116971)
}
