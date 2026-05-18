//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 738/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk738<F: Float>(t10012: F, t8669: F, t2101: F, t2925: F, t313: F, t769: F, t9014: F, t321: F, t1: F, t10810: F, t2021: F, t22623: F, t8502: F) -> (F, F, F, F, F, F, F, F) {
    let t24549 = t10012 * t8669;
    let t24660 = t2101 * t2925;
    let t24661 = t313 * t24660;
    let t24799 = t769 * t9014;
    let t24884 = t321 * t2925;
    let t24885 = t24884 * t1;
    let t24968 = t2021 * t10810;
    let t25070 = t22623 * t8502;
    (t24549, t24660, t24661, t24799, t24884, t24885, t24968, t25070)
}
