//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1046/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1046<F: Float>(t35244: F, t35271: F, t35359: F, t35418: F, t35425: F, t35456: F, t35471: F, t35486: F, t35529: F, t35560: F, t35587: F, t35643: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37449 = F::cast_from(0.85748036236139473944e-3_f64) * t35244;
    let t37464 = F::cast_from(0.21437009059034868486e-3_f64) * t35271;
    let t37504 = F::cast_from(0.39221875e0_f64) * t35359;
    let t37538 = F::cast_from(0.66040993808168719343e-1_f64) * t35418;
    let t37541 = F::cast_from(0.95275595817932748827e-2_f64) * t35425;
    let t37559 = F::cast_from(0.21437009059034868486e-2_f64) * t35456;
    let t37565 = F::cast_from(0.19055119163586549766e-2_f64) * t35471;
    let t37570 = F::cast_from(0.25724410870841842184e-2_f64) * t35486;
    let t37591 = F::cast_from(0.68598428988911579156e-2_f64) * t35529;
    let t37610 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t35560;
    let t37622 = F::cast_from(0.85748036236139473944e-3_f64) * t35587;
    let t37645 = F::cast_from(13.0_f64) / F::cast_from(48.0_f64) * t35643;
    (t37449, t37464, t37504, t37538, t37541, t37559, t37565, t37570, t37591, t37610, t37622, t37645)
}
