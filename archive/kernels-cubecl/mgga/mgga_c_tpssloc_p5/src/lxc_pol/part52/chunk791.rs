//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 791/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk791<F: Float>(t2132: F, t52: F, t2136: F, t6729: F, t1184: F, t460: F, t2147: F, t478: F, t2131: F, t6739: F) -> (F, F, F, F, F, F) {
    let t7313 = t2132 * t52;
    let t7315 = F::cast_from(0.10093189023535097714e-3_f64) * t7313 * t2136;
    let t7316 = t2132 * t6729;
    let t7319 = t1184 * t460;
    let t7320 = t2147 * t478;
    let t7321 = t7319 * t7320;
    let t7324 = t2131 * t6739;
    (t7315, t7316, t7319, t7320, t7321, t7324)
}
