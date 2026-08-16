//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2072/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2072<F: Float>(t90987: F, t22642: F, t22690: F, t26395: F, t22863: F, t7737: F, t26448: F, t90497: F, t215: F, t6916: F, t225: F, t3787: F) -> (F, F, F, F, F, F) {
    let t90988 = F::cast_from(0.82246703342411321824e-2_f64) * t90987;
    let t90993 = t22642 * t22690 * t26395;
    let t91000 = t22863 * t7737;
    let t91002 = t90497 * t26448;
    let t91004 = t6916 * t215;
    let t91005 = t225 * t3787;
    (t90988, t90993, t91000, t91002, t91004, t91005)
}
