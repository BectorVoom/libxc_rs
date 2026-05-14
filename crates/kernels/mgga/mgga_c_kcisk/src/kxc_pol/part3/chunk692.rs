//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 692/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk692<F: Float>(t1060: F, t11612: F, t3293: F, t5136: F, t1850: F, t3290: F, t4597: F, t967: F, t10487: F, t167: F, t11458: F, t1049: F, t695: F, t10399: F, t10441: F, t10449: F, t11495: F, t1809: F, t5089: F, t5168: F) -> (F,) {
    let t11613 = t11612 * t1060;
    let t11615 = t5136 * t3293;
    let t11623 = t1850 * t3290;
    let t11625 = t967 * t4597;
    let t11626 = t11625 * t3290;
    let t11630 = t167 * t10487;
    let t11633 = 0.71734315950379065738e-1 * t11458;
    let t11634 = t1049 * t695;
    let t11635 = 0.62154466893555682512e-3 * t11634;
    let t11636 = 0.11955719325063177623e-1 * t1809 * t10449 - 0.93231700340333523768e-3 * t11613 + 0.31077233446777841256e-3 * t11615 - 0.5179538907796306876e-4 * t1850 * t10449 - 0.71734315950379065738e-1 * t5089 * t10399 + 0.46615850170166761884e-3 * t5168 * t10399 + 0.71734315950379065738e-1 * t11623 - 0.93231700340333523768e-3 * t11626 + 0.71734315950379065738e-1 * t11495 * t10441 - 0.62154466893555682512e-3 * t11630 * t10441 + t11633 - t11635;
    (t11636,)
}
