//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2146/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2146(t22751: f64, t28139: f64, t28159: f64, t6897: f64, t794: f64, t19763: f64, t1992: f64, t6976: f64, t1336: f64, t19735: f64, t22873: f64, t26403: f64, t26459: f64, t5234: f64, t5334: f64, t6388: f64, t6415: f64, t81105: f64, t90971: f64, t90984: f64, t90988: f64, t93595: f64, t97079: f64, t97083: f64, t97087: f64, t97091: f64, t97095: f64, t97106: f64) -> f64 {
    let t97108 = t22751 * t28139;
    let t97111 = t6897 * t794 * t28159;
    let t97114 = t1992 * t6976 * t19763;
    let t97116 = -0.82246703342411321825e-2_f64 * t97079 + 0.3289868133696452873e-1_f64 * t97083 + 0.3289868133696452873e-1_f64 * t97087 + 0.3289868133696452873e-1_f64 * t97091 - 2.0_f64 * t5234 * t26459 + t90971 + 0.76763589786250567037e-1_f64 * t97095 + 4.0_f64 * t5334 * t26403 * t19735 - t1336 * t22873 * t6415 + t93595 + t90984 - t90988 + 2.0_f64 * t1336 * t81105 * t6388 + 0.3289868133696452873e-1_f64 * t97106 + 0.38381794893125283518e-1_f64 * t97108 - 0.41123351671205660912e-2_f64 * t97111 - 0.82246703342411321825e-2_f64 * t97114;
    t97116
}
