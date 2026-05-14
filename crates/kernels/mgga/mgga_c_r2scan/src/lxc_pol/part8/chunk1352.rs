//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1352/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1352<F: Float>(t10156: F, t1584: F, t19858: F, t20721: F, t22997: F, t24463: F, t2573: F, t2651: F, t3081: F, t32532: F, t32923: F, t32927: F, t32930: F, t32936: F, t33063: F, t481: F, t5108: F, t5109: F, t551: F, t552: F, t560: F, t574: F, t6106: F, t7566: F, t9202: F, t9981: F) -> (F,) {
    let t33068 = -0.17465477326173296717e-1 * t32923 - 0.22084125774650235182e1 * t24463 - 0.17465477326173296717e-1 * t32927 + 0.20803732176130244552e1 * t32930 - 0.39006997830244208535e0 * t5108 * t5109 * t32532 * t2573 - 0.15602799132097683414e1 * t6106 * t5109 * t32936 - 0.13002332610081402845e0 * t7566 * t3081 + 0.10401866088065122276e1 * t22997 * t551 * t552 * t9981 * t560 + 0.7801399566048841707e0 * t20721 * t551 * t552 * t9981 * t481 - 0.13002332610081402845e0 * t2651 * t9202 - 0.43341108700271342816e-1 * t1584 * t10156 - 0.43341108700271342816e-1 * t574 * t551 * t552 * t33063 + t19858;
    (t33068,)
}
