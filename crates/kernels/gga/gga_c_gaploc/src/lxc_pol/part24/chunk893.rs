//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 893/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk893<F: Float>(t1891: F, t9014: F, t1901: F, t8729: F, t1025: F, t1841: F, t1897: F, t1908: F, t2508: F, t2933: F, t2960: F, t5227: F, t5269: F, t5524: F, t7129: F, t7137: F, t7299: F, t7303: F, t7309: F, t7315: F, t7318: F, t8991: F, t8994: F, t8997: F, t9000: F, t9003: F, t9007: F) -> F {
    let t9015 = t9014 * t1891;
    let t9020 = t1901 * t8729;
    let t9030 = F::cast_from(0.34180116578409885707e-2_f64) * t1908 * t1025 + F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t8991 + F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t8994 + F::cast_from(0.30762104920568897134e-1_f64) * t2508 * t8997 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t9000 - F::cast_from(0.46143157380853345701e-1_f64) * t2508 * t9003 - F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t9007 + F::cast_from(0.8545029144602471425e-3_f64) * t5524 * t2933 - F::cast_from(0.17090058289204942853e-2_f64) * t5227 * t2933 + F::cast_from(0.92286314761706691403e-1_f64) * t2508 * t9015 + F::cast_from(0.30762104920568897134e-1_f64) * t7129 * t2960 - F::cast_from(0.15381052460284448567e-1_f64) * t5269 * t9020 + F::cast_from(0.41016139894091862847e-1_f64) * t7137 * t2960 + F::cast_from(0.1281754371690370714e-2_f64) * t7299 - F::cast_from(0.1922631557535556071e-2_f64) * t7303 - F::cast_from(0.1281754371690370714e-2_f64) * t7309 + F::cast_from(0.1281754371690370714e-2_f64) * t7315 + F::cast_from(0.64087718584518535698e-3_f64) * t7318;
    t9030
}
