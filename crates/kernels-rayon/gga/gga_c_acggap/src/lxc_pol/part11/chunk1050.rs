//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1050/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1050(t34429: f64, t30570: f64, t30582: f64, t2278: f64, t7600: f64, t2290: f64, t7610: f64, t30541: f64, t30544: f64, t30559: f64, t30561: f64, t30565: f64, t30569: f64, t30577: f64, t34413: f64, t34414: f64, t34417: f64, t34422: f64, t34424: f64, t34427: f64) -> f64 {
    let t34430 = 0.10718504529517434243e-2_f64 * t34429;
    let t34431 = 0.18868855373762491241e-1_f64 * t30570;
    let t34432 = 0.12579236915841660827e-2_f64 * t30582;
    let t34433 = t7600 * t2278;
    let t34435 = t7610 * t2290;
    let t34437 = t34413 - t34414 + 0.80031500487063509016e-2_f64 * t30541 - 0.12862205435420921092e-1_f64 * t30544 - t34417 + 0.83861579438944405513e-3_f64 * t30559 + 0.20965394859736101378e-2_f64 * t30561 + 0.28582678745379824648e-3_f64 * t30565 - t34422 - t34424 / 32.0_f64 - t34427 / 64.0_f64 - t30569 - t34430 - t34431 + t30577 + t34432 - 77.0_f64 / 1728.0_f64 * t34433 + 0.47172138434406228102e-3_f64 * t34435;
    t34437
}
