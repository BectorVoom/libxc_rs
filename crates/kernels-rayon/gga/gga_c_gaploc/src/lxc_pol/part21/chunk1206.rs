//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1206/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1206(t21556: f64, t3440: f64, t2554: f64, t7064: f64, t8871: f64, t1897: f64, t7671: f64, t8637: f64, t3437: f64, t7211: f64, t10749: f64, t2549: f64) -> (f64, f64, f64, f64, f64) {
    let t32400 = 0.6152420984113779427e-1_f64 * t21556 * t3440;
    let t32407 = t7064 * t8871 * t2554;
    let t32408 = 0.64087718584518535698e-3_f64 * t32407;
    let t32411 = 0.46143157380853345702e-1_f64 * t1897 * t8637 * t7671;
    let t32412 = t7211 * t3437;
    let t32413 = 0.32043859292259267849e-3_f64 * t32412;
    let t32414 = t2549 * t10749;
    (t32400, t32408, t32411, t32413, t32414)
}
