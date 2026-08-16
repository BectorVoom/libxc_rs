//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1290/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1290(t13435: f64, t7003: f64, t2322: f64, t25856: f64, t25188: f64, t7313: f64, t508: f64, t651: f64, t94991: f64, t2014: f64, t25177: f64, t7312: f64) -> (f64, f64, f64, f64, f64) {
    let t95040 = 12.0_f64 * t13435 * t7003;
    let t95042 = 6.0_f64 * t2322 * t25856;
    let t95046 = 3.0_f64 * t25188 * t7313;
    let t95049 = 2.0_f64 * t651 * t508 * t94991;
    let t95056 = 6.0_f64 * t2014 * t7312 * t25177;
    (t95040, t95042, t95046, t95049, t95056)
}
