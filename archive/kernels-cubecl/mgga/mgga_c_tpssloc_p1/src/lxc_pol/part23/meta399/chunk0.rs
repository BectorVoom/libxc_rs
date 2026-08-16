//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1208/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1208<F: Float>(t2860: F, t5737: F, t10813: F, t5758: F, t2841: F, t5689: F, t2403: F, t5720: F, t5723: F, t5717: F, t2929: F, t5769: F) -> (F, F, F, F, F, F, F) {
    let t59920 = t5737 * t2860;
    let t59941 = t5758 * t10813;
    let t59959 = t5689 * t2841;
    let t60168 = t2403 * t5720;
    let t60173 = t2403 * t5723;
    let t60204 = t2403 * t5717;
    let t60343 = t5769 * t2929;
    (t59920, t59941, t59959, t60168, t60173, t60204, t60343)
}
