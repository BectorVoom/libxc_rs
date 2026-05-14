//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 744/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk744<F: Float>(t10892: F, t1980: F, t1858: F, t3431: F, t2101: F, t1890: F, t3487: F, t107: F, t10809: F, t787: F, t5241: F, t16687: F, t19: F, t60: F, t822: F, t16692: F, t201: F) -> (F, F, F, F, F, F, F, F) {
    let t33206 = t1980 * t10892;
    let t33232 = t1858 * t3431;
    let t33285 = t2101 * t3431;
    let t33289 = t1890 * t3487;
    let t33294 = t787 * t10809 * t107;
    let t33308 = t5241 * t3487;
    let t33331 = t822 * t16687 * t19 * t60;
    let t33332 = t201 * t16692;
    (t33206, t33232, t33285, t33289, t33294, t33308, t33331, t33332)
}
