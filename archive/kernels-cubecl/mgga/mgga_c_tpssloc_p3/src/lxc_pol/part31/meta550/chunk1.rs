//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1778/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1778<F: Float>(t794: F, t852: F, t23030: F, t23208: F, t1882: F, t81686: F, t9537: F, t213: F, t225: F, t6556: F, t81632: F, t23012: F, t6573: F) -> (F, F, F, F, F, F) {
    let t82133 = t794 * t852;
    let t82147 = t23030 * t23208;
    let t82153 = t81686 * t9537 * t1882;
    let t82159 = t213 * t852 * t225;
    let t82209 = t81632 * t6556;
    let t82211 = t23012 * t6573;
    (t82133, t82147, t82153, t82159, t82209, t82211)
}
