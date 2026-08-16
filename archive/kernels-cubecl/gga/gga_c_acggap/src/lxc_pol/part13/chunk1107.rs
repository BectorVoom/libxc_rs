//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1107/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1107<F: Float>(t35194: F, t1165: F, t21118: F, t7426: F, t8600: F, t5209: F, t7822: F, t30990: F, t30991: F, t30998: F, t31002: F, t31003: F, t31016: F, t31021: F, t31023: F, t35167: F, t35172: F, t35176: F, t35180: F, t35184: F, t35186: F, t35191: F) -> F {
    let t35195 = F::cast_from(0.18868855373762491241e-2_f64) * t35194;
    let t35198 = t7426 * t1165 * t8600 * t21118;
    let t35199 = F::cast_from(0.37737710747524982482e-2_f64) * t35198;
    let t35200 = t7822 * t5209;
    let t35202 = -t30990 - F::cast_from(0.64311027177104605458e-3_f64) * t30991 - t35167 - F::cast_from(0.20965394859736101378e-3_f64) * t30998 + t31002 - F::cast_from(0.85748036236139473944e-3_f64) * t31003 + t31016 - t31021 + t31023 - F::cast_from(0.62896184579208304136e-3_f64) * t35172 - F::cast_from(0.41930789719472202757e-3_f64) * t35176 + F::cast_from(0.10718504529517434243e-3_f64) * t35180 - F::cast_from(0.20965394859736101378e-3_f64) * t35184 - F::cast_from(0.64311027177104605458e-2_f64) * t35186 + t35191 - t35195 + t35199 + F::cast_from(0.34299214494455789578e-2_f64) * t35200;
    t35202
}
