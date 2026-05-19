//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 838/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk838<F: Float>(t8129: F, t8453: F, t8459: F, t8478: F, t8492: F, t8494: F, t8507: F, t8509: F, t8527: F, t8529: F, t8531: F, t8533: F, t8546: F, t8556: F, t8558: F, t8572: F, t8574: F, t8578: F, t9522: F) -> F {
    let t9853 = F::cast_from(0.17149607247227894789e-2_f64) * t8453 + t8129 - F::cast_from(0.31448092289604152068e-2_f64) * t8459 + F::cast_from(0.12579236915841660828e-2_f64) * t8478 + F::cast_from(0.12579236915841660828e-2_f64) * t8492 - F::cast_from(0.85748036236139473944e-3_f64) * t8494 - F::cast_from(0.57165357490759649296e-3_f64) * t8507 + F::cast_from(0.31448092289604152068e-2_f64) * t8509 + F::cast_from(0.17149607247227894789e-2_f64) * t9522 - F::cast_from(0.28582678745379824648e-3_f64) * t8527 + F::cast_from(0.62896184579208304138e-3_f64) * t8529 + F::cast_from(0.25724410870841842184e-2_f64) * t8531 - F::cast_from(0.21437009059034868486e-2_f64) * t8533 - F::cast_from(0.14291339372689912324e-2_f64) * t8546 + F::cast_from(0.20965394859736101379e-2_f64) * t8556 - F::cast_from(0.12579236915841660828e-2_f64) * t8558 - F::cast_from(0.83861579438944405517e-3_f64) * t8572 - F::cast_from(0.17149607247227894789e-2_f64) * t8574 + F::cast_from(0.18868855373762491241e-2_f64) * t8578;
    t9853
}
