//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1071/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1071<F: Float>(t3973: F, t6485: F, t1580: F, t3936: F, t4374: F, t2326: F, t442: F, t4401: F, t1588: F, t5675: F, t13917: F, t6476: F, t18953: F, t4391: F, t3952: F, t14935: F, t19033: F) -> (F, F, F, F, F, F, F, F) {
    let t21517 = t3973 * t6485;
    let t21519 = 0.11993859144118211476e-1 * t1580 * t21517;
    let t21524 = t3936 * t4374;
    let t21525 = t2326 * t442;
    let t21526 = t21525 * t4401;
    let t21527 = t21524 * t21526;
    let t21530 = t3936 * t1588;
    let t21531 = t5675 * t4401;
    let t21532 = t21530 * t21531;
    let t21536 = t13917 * t6476;
    let t21537 = t1580 * t21536;
    let t21539 = t4391 * t18953;
    let t21540 = t3952 * t21539;
    let t21543 = t14935 * t19033;
    (t21519, t21526, t21527, t21531, t21532, t21537, t21540, t21543)
}
