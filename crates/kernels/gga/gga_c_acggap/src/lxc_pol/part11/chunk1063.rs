//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1063/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1063<F: Float>(t2030: F, t4586: F, t7815: F, t30640: F, t30645: F, t30647: F, t30649: F, t30653: F, t30658: F, t34566: F, t34571: F, t34575: F, t34578: F, t34582: F, t34586: F, t34590: F, t34593: F, t34595: F, t34598: F, t34601: F) -> F {
    let t34604 = t2030 * t7815 * t4586;
    let t34606 = -t34566 - F::new(0.10718504529517434243e-3) * t30640 - F::new(0.17149607247227894789e-2) * t30645 + t34571 + F::new(0.12862205435420921092e-2) * t30647 + F::new(0.64311027177104605458e-3) * t30649 - F::new(0.47172138434406228102e-3) * t30653 - t34575 - t30658 + F::new(0.31448092289604152068e-2) * t34578 - F::new(0.12579236915841660827e-2) * t34582 + F::new(0.18868855373762491241e-2) * t34586 - F::new(0.85748036236139473944e-3) * t34590 - t34593 + t34595 / F::new(16.0) + t34598 / F::new(32.0) + t34601 / F::new(64.0) + t34604 / F::new(128.0);
    t34606
}
