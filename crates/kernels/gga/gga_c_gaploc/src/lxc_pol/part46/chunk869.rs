//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 869/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk869<F: Float>(t33331: F, t33332: F, t43494: F, t2660: F, t33576: F, t10930: F, t10931: F, t43508: F, t10893: F, t2628: F, t10914: F, t10915: F, t43586: F, t7572: F, t7573: F, t10811: F, t9961: F) -> (F, F, F, F, F, F, F) {
    let t44064 = 0.13803453343411469884e3 * t33331 * t33332 * t43494;
    let t44065 = t33576 * t2660;
    let t44069 = 0.27606906686822939767e2 * t10930 * t10931 * t43508;
    let t44070 = t10893 * t2628;
    let t44074 = 0.21450293971110256001e1 * t10914 * t10915 * t43508;
    let t44076 = t7572 * t7573 * t43586;
    let t44079 = 0.85801175884441024006e1 * t10811 * t9961;
    (t44064, t44065, t44069, t44070, t44074, t44076, t44079)
}
