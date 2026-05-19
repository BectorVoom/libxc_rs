//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 886/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk886<F: Float>(t1023: F, t1853: F, t779: F, t2936: F, t5836: F, t1843: F, t8529: F, t1841: F, t1897: F, t2508: F, t2909: F, t2912: F, t2937: F, t2955: F, t5227: F, t5269: F, t5288: F, t5293: F, t5524: F, t7129: F, t7137: F, t7141: F, t8912: F, t8919: F, t8926: F, t8929: F, t8932: F, t8939: F) -> (F, F) {
    let t8942 = t1023 * t1853;
    let t8943 = t779 * t8942;
    let t8946 = t2936 * t5836;
    let t8950 = t1843 * t8529;
    let t8957 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t8912 - F::cast_from(0.15381052460284448567e-1_f64) * t5288 * t2909 + F::cast_from(0.15381052460284448567e-1_f64) * t7129 * t2912 - F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t8919 - F::cast_from(0.20508069947045931424e-1_f64) * t5293 * t2909 + F::cast_from(0.20508069947045931424e-1_f64) * t7137 * t2912 - F::cast_from(0.46143157380853345702e-1_f64) * t2508 * t8926 + F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t8929 + F::cast_from(0.15381052460284448567e-1_f64) * t5269 * t8932 - F::cast_from(0.61524209841137794271e-1_f64) * t7137 * t2937 - F::cast_from(0.46143157380853345702e-1_f64) * t7129 * t2937 - F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t8939 - F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t8943 + F::cast_from(0.46143157380853345702e-1_f64) * t1897 * t8946 + F::cast_from(0.19938401337405766662e-2_f64) * t7141 + F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t8950 - F::cast_from(0.8545029144602471425e-3_f64) * t5524 * t2955 + F::cast_from(0.17090058289204942853e-2_f64) * t5227 * t2955;
    (t8942, t8957)
}
