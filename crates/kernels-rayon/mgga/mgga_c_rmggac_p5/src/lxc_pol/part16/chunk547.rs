//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 547/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk547(t4443: f64, t671: f64, t674: f64, t1993: f64, t2185: f64, t1997: f64, t4179: f64, t6: f64, t220: f64, t211: f64, t483: f64, t1965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7407 = t671 * t4443;
    let t7408 = t7407 * t674;
    let t7414 = t1993 * t2185;
    let t7415 = t7414 * t1997;
    let t7417 = t6 * t4179;
    let t7418 = t220 * t7417;
    let t7427 = t211 * t483;
    let t7428 = t1965 * t7427;
    (t7407, t7408, t7414, t7415, t7417, t7418, t7427, t7428)
}
