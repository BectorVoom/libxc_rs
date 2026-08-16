//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 773/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk773(t7317: f64, t943: f64, t1841: f64, t1850: f64, t1897: f64, t2508: f64, t2538: f64, t2573: f64, t5227: f64, t5288: f64, t5293: f64, t5524: f64, t7251: f64, t7255: f64, t7260: f64, t7268: f64, t7277: f64, t7281: f64, t7286: f64, t7293: f64, t7299: f64, t7303: f64, t7306: f64, t7309: f64, t7315: f64) -> (f64, f64) {
    let t7318 = t943 * t7317;
    let t7320 = 0.15381052460284448567e-1_f64 * t1897 * t7251 + 0.76905262301422242837e-2_f64 * t1897 * t7255 + 0.30762104920568897134e-1_f64 * t2508 * t7260 + 0.20508069947045931424e-1_f64 * t5293 * t2573 + 0.15381052460284448567e-1_f64 * t5288 * t2573 + 0.17090058289204942853e-2_f64 * t1850 * t7268 + 0.8545029144602471425e-3_f64 * t5524 * t2538 - 0.17090058289204942853e-2_f64 * t5227 * t2538 - 0.17090058289204942853e-2_f64 * t1841 * t7277 - 0.17090058289204942853e-2_f64 * t1841 * t7281 + 0.51270174867614828558e-2_f64 * t1841 * t7286 - 0.34180116578409885705e-2_f64 * t1841 * t7293 + 0.64087718584518535698e-3_f64 * t7299 - 0.96131577876777803547e-3_f64 * t7303 - 0.30762104920568897134e-1_f64 * t1897 * t7306 - 0.64087718584518535698e-3_f64 * t7309 + 0.64087718584518535698e-3_f64 * t7315 + 0.32043859292259267849e-3_f64 * t7318;
    (t7318, t7320)
}
