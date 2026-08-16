//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3295/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3295<F: Float>(t61519: F, t62429: F, t62431: F, t62435: F, t62439: F, t62441: F, t62443: F, t62445: F, t62453: F, t62458: F, t62460: F, t828: F, t851: F, t855: F) -> F {
    let t62462 = -F::cast_from(0.28582678745379824648e-3_f64) * t62429 - F::cast_from(0.27104001498285508387e-2_f64) * t62431 + F::cast_from(0.17149607247227894789e-2_f64) * t62435 - F::cast_from(0.57165357490759649296e-3_f64) * t62439 + F::cast_from(0.80031500487063509014e-2_f64) * t62441 + F::cast_from(0.15244095330869239812e-3_f64) * t62443 - F::cast_from(0.76220476654346199061e-4_f64) * t62445 - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t855 * t828 * t61519 - F::cast_from(0.10164000561857065645e-3_f64) * t62453 + F::cast_from(0.14291339372689912324e-4_f64) * t62458 + F::cast_from(0.32012600194825403606e-1_f64) * t62460;
    t62462
}
