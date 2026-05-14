//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 819/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk819<F: Float>(t3293: F, t696: F, t1849: F, t213: F, t1060: F, t5136: F, t1850: F, t3290: F, t4597: F, t967: F, t10487: F, t167: F, t11458: F, t1049: F, t695: F, t1333: F, t5177: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11607 = t696 * t3293;
    let t11612 = t213 * t1849;
    let t11613 = t11612 * t1060;
    let t11615 = t5136 * t3293;
    let t11623 = t1850 * t3290;
    let t11625 = t967 * t4597;
    let t11626 = t11625 * t3290;
    let t11630 = t167 * t10487;
    let t11633 = 0.71734315950379065738e-1 * t11458;
    let t11634 = t1049 * t695;
    let t11635 = 0.62154466893555682512e-3 * t11634;
    let t11652 = t1333 * t5177;
    (t11607, t11612, t11613, t11615, t11623, t11625, t11626, t11630, t11633, t11634, t11635, t11652)
}
