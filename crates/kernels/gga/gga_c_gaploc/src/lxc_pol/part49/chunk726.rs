//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 726/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk726<F: Float>(t769: F, t9014: F, t2925: F, t321: F, t1: F, t10810: F, t2021: F, t22623: F, t8502: F, t8774: F, t10007: F, t8669: F, t5750: F, t10555: F, t161: F, t197: F, t2754: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24799 = t769 * t9014;
    let t24884 = t321 * t2925;
    let t24885 = t24884 * t1;
    let t24968 = t2021 * t10810;
    let t25070 = t22623 * t8502;
    let t25198 = t2021 * t8774;
    let t25359 = t10007 * t8669;
    let t25405 = t5750 * t2925;
    let t25718 = t10555 * t161;
    let t25760 = t197 * t2754;
    (t24799, t24884, t24885, t24968, t25070, t25198, t25359, t25405, t25718, t25760)
}
