//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 975/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk975<F: Float>(t7839: F, t8481: F, t30534: F, t30536: F, t30547: F, t2020: F, t8942: F, t1988: F, t8536: F, t30570: F, t30582: F, t2278: F, t7600: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34409 = t7839 * t8481;
    let t34413 = F::new(0.19055119163586549766e-2) * t30534;
    let t34414 = F::new(0.18868855373762491241e-2) * t30536;
    let t34417 = F::new(0.51448821741683684368e-2) * t30547;
    let t34421 = t2020 * t8942;
    let t34429 = t1988 * t8536;
    let t34431 = F::new(0.18868855373762491241e-1) * t30570;
    let t34432 = F::new(0.12579236915841660827e-2) * t30582;
    let t34433 = t7600 * t2278;
    (t34409, t34413, t34414, t34417, t34421, t34429, t34431, t34432, t34433)
}
