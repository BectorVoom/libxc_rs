//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 459/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk459<F: Float>(t313: F, t8637: F, t1022: F, t701: F, t739: F, t8502: F, t2610: F, t7290: F, t321: F, t107: F, t787: F, t1858: F, t1: F, t2021: F, t1890: F, t2925: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8638 = t313 * t8637;
    let t8669 = t1022 * t701;
    let t8670 = t739 * t8669;
    let t8682 = t739 * t8502;
    let t8756 = t2610 * t8669;
    let t8769 = t7290 * t8502;
    let t8773 = t321 * t1022;
    let t8774 = t8773 * t107;
    let t8775 = t787 * t8774;
    let t8788 = t1858 * t1022;
    let t8792 = t8773 * t1;
    let t8793 = t2021 * t8792;
    let t8802 = t1890 * t2925;
    (t8638, t8669, t8670, t8682, t8756, t8769, t8773, t8774, t8775, t8788, t8792, t8793, t8802)
}
