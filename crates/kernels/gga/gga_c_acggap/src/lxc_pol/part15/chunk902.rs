//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 902/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk902<F: Float>(t31277: F, t31279: F, t1988: F, t8486: F, t1967: F, t8838: F, t31285: F, t4360: F, t7741: F, t31312: F, t31316: F, t31322: F, t13287: F, t31057: F, t33953: F, t5122: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35507 = 0.3973125e0 * t31277;
    let t35508 = 0.264875e0 * t31279;
    let t35513 = t1988 * t8486;
    let t35515 = t1967 * t8838;
    let t35527 = 0.10718504529517434243e-2 * t31285;
    let t35529 = t7741 * t4360;
    let t35538 = 0.85748036236139473944e-3 * t31312;
    let t35539 = 0.12579236915841660827e-2 * t31316;
    let t35541 = 0.85748036236139473944e-3 * t31322;
    let t35549 = t31057 * t13287 * t33953 * t5122;
    (t35507, t35508, t35513, t35515, t35527, t35529, t35538, t35539, t35541, t35549)
}
