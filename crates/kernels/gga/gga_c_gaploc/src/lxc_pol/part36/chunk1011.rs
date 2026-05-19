//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1011/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1011<F: Float>(t33331: F, t33332: F, t43494: F, t2660: F, t33576: F, t10930: F, t10931: F, t43508: F, t10893: F, t2628: F, t10914: F, t10915: F) -> (F, F, F, F, F) {
    let t44064 = F::cast_from(0.13803453343411469884e3_f64) * t33331 * t33332 * t43494;
    let t44065 = t33576 * t2660;
    let t44069 = F::cast_from(0.27606906686822939767e2_f64) * t10930 * t10931 * t43508;
    let t44070 = t10893 * t2628;
    let t44074 = F::cast_from(0.21450293971110256001e1_f64) * t10914 * t10915 * t43508;
    (t44064, t44065, t44069, t44070, t44074)
}
