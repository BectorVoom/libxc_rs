//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 488/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk488<F: Float>(t313: F, t8637: F, t1022: F, t701: F, t739: F, t8502: F, t107: F, t2931: F, t2610: F, t7290: F, t321: F, t787: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8638 = t313 * t8637;
    let t8669 = t1022 * t701;
    let t8682 = t739 * t8502;
    let t8748 = t2931 * t107;
    let t8756 = t2610 * t8669;
    let t8769 = t7290 * t8502;
    let t8773 = t321 * t1022;
    let t8774 = t8773 * t107;
    let t8775 = t787 * t8774;
    (t8638, t8669, t8682, t8748, t8756, t8769, t8773, t8774, t8775)
}
