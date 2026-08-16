//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1065/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1065<F: Float>(t13518: F, t2142: F, t13609: F, t840: F, t13628: F, t1161: F, t35889: F, t829: F, t830: F, t13096: F, t34857: F, t1105: F, t12232: F) -> (F, F, F, F, F, F) {
    let t46615 = t13518 * t2142;
    let t46635 = t840 * t13609;
    let t46637 = t840 * t13628;
    let t46639 = t35889 * t1161;
    let t46641 = t829 * t830 * t46639;
    let t46650 = t34857 * t13096;
    let t46654 = t12232 * t1105;
    (t46615, t46635, t46637, t46641, t46650, t46654)
}
