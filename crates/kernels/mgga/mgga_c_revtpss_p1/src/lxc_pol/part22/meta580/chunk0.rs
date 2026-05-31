//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2437/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2437<F: Float>(t18281: F, t190: F, t706: F, t14441: F, t10593: F, t10597: F, t189: F, t5819: F, t606: F, t14330: F, t10608: F, t4308: F, t4311: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18569 = t190 * t18281;
    let t18571 = F::cast_from(4.0_f64) * t706 * t18569;
    let t18572 = F::cast_from(8.0_f64) * t14441;
    let t18573 = F::cast_from(0.5848223622634646207e0_f64) * t10593;
    let t18574 = F::cast_from(0.17315859105681463759e2_f64) * t10597;
    let t18575 = t189 * t5819;
    let t18576 = t18575 * t606;
    let t18578 = F::cast_from(24.0_f64) * t14330 * t18576;
    let t18579 = F::cast_from(0.11696447245269292414e1_f64) * t10608;
    let t18581 = F::cast_from(8.0_f64) * t4311 * t4308;
    (t18569, t18571, t18572, t18573, t18574, t18575, t18576, t18578, t18579, t18581)
}
