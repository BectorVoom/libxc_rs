//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 995/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk995<F: Float>(t35210: F, t30219: F, t8515: F, t1345: F, t1992: F, t30154: F, t7586: F, t1535: F, t4180: F, t7646: F, t1181: F, t30327: F, t4358: F, t599: F) -> (F, F, F, F, F, F) {
    let t35211 = F::cast_from(0.94344276868812456204e-2_f64) * t35210;
    let t35212 = t30219 * t8515;
    let t35213 = F::cast_from(0.21437009059034868486e-2_f64) * t35212;
    let t35225 = t1992 * t1345;
    let t35227 = t30154 * t7586 * t35225;
    let t35228 = F::cast_from(0.14291339372689912324e-2_f64) * t35227;
    let t35230 = t4180 * t7646 * t1535;
    let t35231 = F::cast_from(0.17149607247227894789e-2_f64) * t35230;
    let t35238 = t30327 * t1181 * t599 * t4358;
    (t35211, t35213, t35225, t35228, t35231, t35238)
}
