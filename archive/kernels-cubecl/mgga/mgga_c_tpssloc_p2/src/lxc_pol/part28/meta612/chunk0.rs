//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1925/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1925<F: Float>(t22863: F, t7737: F, t26448: F, t90497: F, t215: F, t6916: F, t225: F, t3787: F, t562: F, t16313: F, t22751: F, t26385: F) -> (F, F, F, F, F, F) {
    let t91000 = t22863 * t7737;
    let t91002 = t90497 * t26448;
    let t91004 = t6916 * t215;
    let t91005 = t225 * t3787;
    let t91006 = t91005 * t562;
    let t91008 = t91004 * t91006 * t16313;
    let t91010 = t22751 * t26385;
    (t91000, t91002, t91004, t91005, t91008, t91010)
}
