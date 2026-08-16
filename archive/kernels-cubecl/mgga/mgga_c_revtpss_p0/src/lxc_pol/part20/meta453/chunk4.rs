//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1733/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1733<F: Float>(t3930: F, t9893: F, t3957: F, t9700: F, t1413: F, t547: F, t807: F, t9628: F, t3952: F, t9784: F, t281: F, t39644: F, t40650: F, t550: F) -> (F, F, F, F, F) {
    let t46863 = t3930 * t9893;
    let t46865 = t3957 * t9700;
    let t46877 = t807 * t547 * t1413 * t9628;
    let t46879 = t9784 * t3952;
    let t46885 = F::cast_from(0.47607864835161149081e-7_f64) * t39644 * t547 * t40650 * t550 * t281;
    (t46863, t46865, t46877, t46879, t46885)
}
