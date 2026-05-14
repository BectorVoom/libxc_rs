//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 815/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk815<F: Float>(t565: F, t9463: F, t1632: F, t3016: F, t551: F, t566: F, t2573: F, t8740: F, t5109: F, t8756: F, t2155: F, t9423: F, t2609: F, t7601: F, t3187: F, t6518: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9469 = t565 * t9463;
    let t9476 = t1632 * t3016;
    let t9477 = t551 * t9476;
    let t9478 = t566 * t9477;
    let t9481 = t8740 * t2573;
    let t9482 = t5109 * t9481;
    let t9485 = t5109 * t8756;
    let t9488 = t2155 * t9423;
    let t9490 = t7601 * t2609;
    let t9498 = t6518 * t3187;
    (t9469, t9476, t9478, t9481, t9482, t9485, t9488, t9490, t9498)
}
