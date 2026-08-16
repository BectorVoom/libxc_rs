//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1246/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1246<F: Float>(t24555: F, t953: F, t2797: F, t8182: F, t25560: F, t3881: F, t2606: F, t7433: F, t2668: F, t877: F, t25029: F, t2619: F, t2758: F, t2760: F) -> (F, F, F, F, F, F, F) {
    let t25730 = t953 * t24555;
    let t25740 = t2797 * t8182;
    let t25742 = t3881 * t25560;
    let t25749 = t7433 * t2606;
    let t25751 = t2668 * t25749 * t877;
    let t25753 = t953 * t25029;
    let t25769 = t2758 * t2619 * t2760;
    (t25730, t25740, t25742, t25749, t25751, t25753, t25769)
}
