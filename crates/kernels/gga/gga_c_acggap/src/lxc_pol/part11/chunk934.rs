//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 934/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk934<F: Float>(t30534: F, t30536: F, t30547: F, t2020: F, t8942: F, t5164: F, t7450: F, t7815: F, t2060: F, t5170: F, t1988: F, t8536: F, t30570: F, t30582: F, t2278: F, t7600: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34413 = 0.19055119163586549766e-2 * t30534;
    let t34414 = 0.18868855373762491241e-2 * t30536;
    let t34417 = 0.51448821741683684368e-2 * t30547;
    let t34421 = t2020 * t8942;
    let t34422 = 7.0 / 144.0 * t34421;
    let t34424 = t7450 * t7815 * t5164;
    let t34427 = t2060 * t7815 * t5170;
    let t34429 = t1988 * t8536;
    let t34430 = 0.10718504529517434243e-2 * t34429;
    let t34431 = 0.18868855373762491241e-1 * t30570;
    let t34432 = 0.12579236915841660827e-2 * t30582;
    let t34433 = t7600 * t2278;
    (t34413, t34414, t34417, t34422, t34424, t34427, t34430, t34431, t34432, t34433)
}
