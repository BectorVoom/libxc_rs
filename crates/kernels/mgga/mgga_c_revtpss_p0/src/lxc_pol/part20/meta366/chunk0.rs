//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1338/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1338<F: Float>(t2523: F, t9323: F, t9318: F, t2596: F, t746: F, t9385: F, t760: F, t186: F, t2698: F, t685: F, t755: F, t10326: F, t10599: F, t4401: F) -> (F, F, F, F, F, F, F) {
    let t40092 = t2523 * t9323;
    let t40093 = F::cast_from(0.20779030926817756511e3_f64) * t40092;
    let t40094 = t2523 * t9318;
    let t40095 = F::cast_from(0.14035736694323150897e2_f64) * t40094;
    let t40097 = t2596 * t9385 * t746;
    let t40099 = F::cast_from(0.46785788981077169656e1_f64) * t760 * t40097;
    let t40101 = t685 * t2698 * t186;
    let t40103 = F::cast_from(0.18989649058080861537e-2_f64) * t755 * t40101;
    let t40106 = F::new(48.0) * t4401 * t10599 * t10326;
    (t40093, t40095, t40097, t40099, t40101, t40103, t40106)
}
