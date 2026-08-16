//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 890/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk890(t1891: f64, t9014: f64, t1901: f64, t8729: f64, t1025: f64, t1841: f64, t1897: f64, t1908: f64, t2508: f64, t2933: f64, t2960: f64, t5227: f64, t5269: f64, t5524: f64, t7129: f64, t7137: f64, t7299: f64, t7303: f64, t7309: f64, t7315: f64, t7318: f64, t8991: f64, t8994: f64, t8997: f64, t9000: f64, t9003: f64, t9007: f64) -> f64 {
    let t9015 = t9014 * t1891;
    let t9020 = t1901 * t8729;
    let t9030 = 0.34180116578409885707e-2_f64 * t1908 * t1025 + 0.15381052460284448567e-1_f64 * t1897 * t8991 + 0.76905262301422242837e-2_f64 * t1897 * t8994 + 0.30762104920568897134e-1_f64 * t2508 * t8997 + 0.15381052460284448567e-1_f64 * t2508 * t9000 - 0.46143157380853345701e-1_f64 * t2508 * t9003 - 0.17090058289204942853e-2_f64 * t1841 * t9007 + 0.8545029144602471425e-3_f64 * t5524 * t2933 - 0.17090058289204942853e-2_f64 * t5227 * t2933 + 0.92286314761706691403e-1_f64 * t2508 * t9015 + 0.30762104920568897134e-1_f64 * t7129 * t2960 - 0.15381052460284448567e-1_f64 * t5269 * t9020 + 0.41016139894091862847e-1_f64 * t7137 * t2960 + 0.1281754371690370714e-2_f64 * t7299 - 0.1922631557535556071e-2_f64 * t7303 - 0.1281754371690370714e-2_f64 * t7309 + 0.1281754371690370714e-2_f64 * t7315 + 0.64087718584518535698e-3_f64 * t7318;
    t9030
}
