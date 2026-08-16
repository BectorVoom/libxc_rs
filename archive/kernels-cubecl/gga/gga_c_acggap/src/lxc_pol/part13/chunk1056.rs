//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1056/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1056<F: Float>(t34506: F, t30984: F, t8458: F, t2268: F, t30456: F, t1562: F, t30948: F, t1444: F, t1992: F, t30154: F, t7586: F, t30596: F, t30607: F, t30611: F, t34482: F, t34484: F, t34485: F, t34489: F, t34492: F, t34497: F, t34499: F, t34501: F, t34502: F, t34504: F) -> F {
    let t34507 = F::cast_from(0.17149607247227894789e-2_f64) * t34506;
    let t34508 = t30984 * t8458;
    let t34510 = t30456 * t2268;
    let t34512 = t30948 * t1562;
    let t34513 = F::cast_from(0.16006300097412701803e-1_f64) * t34512;
    let t34516 = t30154 * t7586 * t1992 * t1444;
    let t34518 = -F::cast_from(0.25724410870841842183e-2_f64) * t34482 + t30596 - t34484 - t34485 + F::cast_from(0.140078125e-1_f64) * t30607 + t34489 - F::cast_from(0.15724046144802076034e-3_f64) * t34492 - F::cast_from(0.25724410870841842184e-2_f64) * t30611 + F::cast_from(0.62896184579208304136e-3_f64) * t34497 - t34499 + t34501 - F::cast_from(0.17149607247227894789e-2_f64) * t34502 - F::cast_from(0.85748036236139473944e-3_f64) * t34504 + t34507 - F::cast_from(0.15724046144802076034e-2_f64) * t34508 + F::cast_from(0.66040993808168719343e-2_f64) * t34510 - t34513 + F::cast_from(0.20965394859736101379e-2_f64) * t34516;
    t34518
}
