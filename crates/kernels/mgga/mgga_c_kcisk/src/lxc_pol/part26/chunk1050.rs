//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1050/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1050<F: Float>(t27906: F, t79: F, t534: F, t1571: F, t8307: F, t1576: F, t8308: F, t13917: F, t8318: F, t1580: F, t6459: F, t6473: F, t21662: F, t21665: F, t21939: F, t21947: F, t21956: F, t2322: F, t4381: F, t541: F, t6507: F, t8328: F) -> (F, F) {
    let t27907 = t79 * t27906;
    let t27908 = t27907 * t534;
    let t27912 = t8307 * t1571;
    let t27915 = t8308 * t1576;
    let t27920 = t13917 * t8318;
    let t27921 = t1580 * t27920;
    let t27925 = t6459 * t6473;
    let t27931 = -0.71963154864709268853e-1 * t27912 * t541 + 0.89953943580886586067e-2 * t27915 + t21939 + 0.11993859144118211476e-1 * t21947 + 0.47975436576472845901e-1 * t4381 * t8328 + 0.39979530480394038253e-2 * t27921 - 0.47975436576472845901e-1 * t21665 * t2322 + 0.59969295720591057377e-2 * t27925 + 0.17990788716177317213e-1 * t21662 * t2322 + 0.10794473229706390328e0 * t6459 * t6507 + t21956;
    (t27908, t27931)
}
