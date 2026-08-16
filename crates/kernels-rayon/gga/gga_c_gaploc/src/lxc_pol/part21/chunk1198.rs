//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1198/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1198(t32171: f64, t10760: f64, t7137: f64, t10718: f64, t10770: f64, t2958: f64, t7112: f64, t2508: f64, t2580: f64, t2530: f64, t8469: f64, t24339: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32172 = 0.64087718584518535698e-3_f64 * t32171;
    let t32185 = 0.6152420984113779427e-1_f64 * t7137 * t10760;
    let t32201 = 0.41016139894091862846e-1_f64 * t7137 * t10718;
    let t32207 = 0.20508069947045931424e-1_f64 * t7137 * t10770;
    let t32210 = t2958 * t7112;
    let t32213 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t32210;
    let t32219 = t8469 * t2530;
    let t32222 = 0.30762104920568897134e-1_f64 * t2508 * t2580 * t32219;
    let t32223 = t24339 * t935;
    (t32172, t32185, t32201, t32207, t32210, t32213, t32219, t32222, t32223)
}
