//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 955/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk955(t32185: f64, t532: f64, t193: f64, t201: f64, t8743: f64, t2752: f64, t32029: f64, t8747: f64, t10143: f64, t40772: f64, t114759: f64, t114814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t116437 = t532 * t32185;
    let t116473 = t193 * t201 * t8743;
    let t116476 = t32029 * t2752;
    let t116481 = t193 * t201 * t8747;
    let t116492 = t8743 * t10143;
    let t116498 = t8747 * t40772;
    let t116514 = 0.25587863262083522346e0_f64 * t114759;
    let t116536 = 0.10417915756705434098e0_f64 * t114814;
    (t116437, t116473, t116476, t116481, t116492, t116498, t116514, t116536)
}
