//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 887/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk887(t1023: f64, t1853: f64, t779: f64, t2936: f64, t5836: f64, t1843: f64, t8529: f64, t1841: f64, t1897: f64, t2508: f64, t2909: f64, t2912: f64, t2937: f64, t2955: f64, t5227: f64, t5269: f64, t5288: f64, t5293: f64, t5524: f64, t7129: f64, t7137: f64, t7141: f64, t8912: f64, t8919: f64, t8926: f64, t8929: f64, t8932: f64, t8939: f64) -> (f64, f64) {
    let t8942 = t1023 * t1853;
    let t8943 = t779 * t8942;
    let t8946 = t2936 * t5836;
    let t8950 = t1843 * t8529;
    let t8957 = 0.15381052460284448567e-1_f64 * t2508 * t8912 - 0.15381052460284448567e-1_f64 * t5288 * t2909 + 0.15381052460284448567e-1_f64 * t7129 * t2912 - 0.15381052460284448567e-1_f64 * t1897 * t8919 - 0.20508069947045931424e-1_f64 * t5293 * t2909 + 0.20508069947045931424e-1_f64 * t7137 * t2912 - 0.46143157380853345702e-1_f64 * t2508 * t8926 + 0.76905262301422242837e-2_f64 * t2508 * t8929 + 0.15381052460284448567e-1_f64 * t5269 * t8932 - 0.61524209841137794271e-1_f64 * t7137 * t2937 - 0.46143157380853345702e-1_f64 * t7129 * t2937 - 0.53833683610995569986e-1_f64 * t2508 * t8939 - 0.15381052460284448567e-1_f64 * t1897 * t8943 + 0.46143157380853345702e-1_f64 * t1897 * t8946 + 0.19938401337405766662e-2_f64 * t7141 + 0.17090058289204942853e-2_f64 * t1841 * t8950 - 0.8545029144602471425e-3_f64 * t5524 * t2955 + 0.17090058289204942853e-2_f64 * t5227 * t2955;
    (t8942, t8957)
}
