//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3124/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3124<F: Float>(t11922: F, t16067: F, t16069: F, t11200: F, t380: F, t16088: F, t3105: F, t4797: F, t15725: F, t15827: F, t11921: F, t16152: F, t247: F, t4837: F) -> (F, F, F, F, F, F) {
    let t55328 = t16067 * t11922 * t16069;
    let t55330 = t11200 * t380;
    let t55331 = t55330 * t16088;
    let t55356 = t4797 * t3105;
    let t55361 = t15725 * t15827;
    let t55367 = t4837 * t247 * t11921 * t16152;
    (t55328, t55330, t55331, t55356, t55361, t55367)
}
