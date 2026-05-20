//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2899/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2899<F: Float>(t3952: F, t9784: F, t281: F, t39644: F, t40650: F, t547: F, t550: F, t2689: F, t9715: F, t40688: F, t46786: F, t9704: F) -> (F, F, F, F, F, F) {
    let t46879 = t9784 * t3952;
    let t46885 = F::cast_from(0.47607864835161149081e-7_f64) * t39644 * t547 * t40650 * t550 * t281;
    let t46886 = t2689 * t9715;
    let t46888 = t40688 * t547;
    let t46889 = t46888 * t46786;
    let t46895 = t2689 * t9704;
    (t46879, t46885, t46886, t46888, t46889, t46895)
}
