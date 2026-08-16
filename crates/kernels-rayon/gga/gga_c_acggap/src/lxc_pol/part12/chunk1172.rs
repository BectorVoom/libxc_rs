//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1172/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1172(t34453: f64, t34468: f64, t34476: f64, t30584: f64, t30586: f64, t30592: f64, t32515: f64, t34446: f64, t34449: f64, t34455: f64, t34457: f64, t34459: f64, t34461: f64, t34463: f64, t34466: f64, t34472: f64, t34478: f64) -> f64 {
    let t37105 = 0.10718504529517434243e-2_f64 * t34453;
    let t37112 = 0.90035438047946447644e-2_f64 * t34468;
    let t37114 = 0.18868855373762491241e-2_f64 * t34476;
    let t37116 = 0.12862205435420921092e-1_f64 * t30584 + 0.51448821741683684368e-2_f64 * t30586 + t32515 - 0.10718504529517434243e-2_f64 * t34446 + 0.19055119163586549766e-1_f64 * t30592 + 0.12579236915841660828e-2_f64 * t34449 + t37105 + 0.68598428988911579156e-2_f64 * t34455 + 0.34299214494455789578e-2_f64 * t34457 - 0.34299214494455789578e-2_f64 * t34459 - 0.68598428988911579156e-2_f64 * t34461 + 0.34299214494455789578e-2_f64 * t34463 - 0.21437009059034868486e-3_f64 * t34466 - t37112 - 0.45017719023973223821e-1_f64 * t34472 + t37114 + 0.11321313224257494745e-1_f64 * t34478;
    t37116
}
