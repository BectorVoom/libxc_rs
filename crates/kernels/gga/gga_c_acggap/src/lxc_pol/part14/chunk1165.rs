//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1165/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1165<F: Float>(t1165: F, t39753: F, t604: F, t7337: F, t31421: F, t35570: F, t35574: F, t35581: F, t35586: F, t35595: F, t35597: F, t35602: F, t35609: F, t35611: F, t35617: F, t37622: F, t40063: F, t40068: F, t40072: F, t40076: F) -> F {
    let t40080 = t7337 * t1165 * t604 * t39753;
    let t40082 = t35570 + F::new(0.21437009059034868486e-2) * t40063 - t35574 + t35581 - t35586 + t37622 + t35595 + t35597 + t35602 + F::new(0.114609375e-1) * t31421 - F::new(0.85748036236139473944e-3) * t40068 - F::new(0.10718504529517434243e-3) * t40072 - F::new(0.15724046144802076034e-3) * t40076 - F::new(0.7862023072401038017e-3) * t40080 + t35609 + t35611 - t35617;
    t40082
}
