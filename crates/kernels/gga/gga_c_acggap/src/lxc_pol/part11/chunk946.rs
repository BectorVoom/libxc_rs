//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 946/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk946<F: Float>(t34592: F, t4578: F, t7450: F, t7815: F, t4483: F, t2030: F, t4582: F, t4586: F, t30640: F, t30645: F, t30647: F, t30649: F, t30653: F, t30658: F, t34566: F, t34571: F, t34575: F, t34578: F, t34582: F, t34586: F, t34590: F) -> (F,) {
    let t34593 = 11.0 / 192.0 * t34592;
    let t34595 = t7450 * t7815 * t4578;
    let t34598 = t7450 * t7815 * t4483;
    let t34601 = t2030 * t7815 * t4582;
    let t34604 = t2030 * t7815 * t4586;
    let t34606 = -t34566 - 0.10718504529517434243e-3 * t30640 - 0.17149607247227894789e-2 * t30645 + t34571 + 0.12862205435420921092e-2 * t30647 + 0.64311027177104605458e-3 * t30649 - 0.47172138434406228102e-3 * t30653 - t34575 - t30658 + 0.31448092289604152068e-2 * t34578 - 0.12579236915841660827e-2 * t34582 + 0.18868855373762491241e-2 * t34586 - 0.85748036236139473944e-3 * t34590 - t34593 + t34595 / 16.0 + t34598 / 32.0 + t34601 / 64.0 + t34604 / 128.0;
    (t34606,)
}
