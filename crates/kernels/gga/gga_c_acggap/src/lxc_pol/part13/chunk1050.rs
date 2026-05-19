//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1050/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1050<F: Float>(t34429: F, t30570: F, t30582: F, t2278: F, t7600: F, t2290: F, t7610: F, t30541: F, t30544: F, t30559: F, t30561: F, t30565: F, t30569: F, t30577: F, t34413: F, t34414: F, t34417: F, t34422: F, t34424: F, t34427: F) -> F {
    let t34430 = F::cast_from(0.10718504529517434243e-2_f64) * t34429;
    let t34431 = F::cast_from(0.18868855373762491241e-1_f64) * t30570;
    let t34432 = F::cast_from(0.12579236915841660827e-2_f64) * t30582;
    let t34433 = t7600 * t2278;
    let t34435 = t7610 * t2290;
    let t34437 = t34413 - t34414 + F::cast_from(0.80031500487063509016e-2_f64) * t30541 - F::cast_from(0.12862205435420921092e-1_f64) * t30544 - t34417 + F::cast_from(0.83861579438944405513e-3_f64) * t30559 + F::cast_from(0.20965394859736101378e-2_f64) * t30561 + F::cast_from(0.28582678745379824648e-3_f64) * t30565 - t34422 - t34424 / F::new(32.0) - t34427 / F::new(64.0) - t30569 - t34430 - t34431 + t30577 + t34432 - F::new(77.0) / F::new(1728.0) * t34433 + F::cast_from(0.47172138434406228102e-3_f64) * t34435;
    t34437
}
