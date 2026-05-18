//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 934/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk934<F: Float>(t11318: F, t2464: F, t2465: F, t587: F, t2365: F, t36211: F, t7025: F, t10430: F, t9263: F, t993: F, t11718: F, t7324: F) -> (F, F, F, F) {
    let t46815 = t587 * t2464 * t2465 * t11318;
    let t46818 = t7025 * t2365 * t36211;
    let t46819 = F::new(0.14896037479937677779e-1) * t46818;
    let t46821 = t9263 * t993 * t10430;
    let t46832 = F::new(2.0) * t7324 * t11718;
    (t46815, t46819, t46821, t46832)
}
