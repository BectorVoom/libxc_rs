//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1022/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1022<F: Float>(t34453: F, t34468: F, t34476: F, t30584: F, t30586: F, t30592: F, t32515: F, t34446: F, t34449: F, t34455: F, t34457: F, t34459: F, t34461: F, t34463: F, t34466: F, t34472: F, t34478: F) -> (F,) {
    let t37105 = 0.10718504529517434243e-2 * t34453;
    let t37112 = 0.90035438047946447644e-2 * t34468;
    let t37114 = 0.18868855373762491241e-2 * t34476;
    let t37116 = 0.12862205435420921092e-1 * t30584 + 0.51448821741683684368e-2 * t30586 + t32515 - 0.10718504529517434243e-2 * t34446 + 0.19055119163586549766e-1 * t30592 + 0.12579236915841660828e-2 * t34449 + t37105 + 0.68598428988911579156e-2 * t34455 + 0.34299214494455789578e-2 * t34457 - 0.34299214494455789578e-2 * t34459 - 0.68598428988911579156e-2 * t34461 + 0.34299214494455789578e-2 * t34463 - 0.21437009059034868486e-3 * t34466 - t37112 - 0.45017719023973223821e-1 * t34472 + t37114 + 0.11321313224257494745e-1 * t34478;
    (t37116,)
}
