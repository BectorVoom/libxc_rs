//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 571/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk571<F: Float>(t1137: F, t1503: F, t3114: F, t355: F, t352: F, t1427: F, t721: F, t1049: F, t1483: F, t1480: F, t3111: F, t1298: F) -> (F, F, F, F, F, F, F) {
    let t4785 = F::new(7.0) / F::new(72.0) * t1137 * t1503;
    let t4794 = t3114 * t355;
    let t4795 = t352 * t4794;
    let t4796 = t1427 * t721;
    let t4797 = t4795 * t4796;
    let t4798 = F::new(0.2445e0) * t4797;
    let t4799 = t1049 * t1483;
    let t4800 = F::new(0.978e0) * t4799;
    let t4804 = t3111 * t1480;
    let t4806 = t355 * t1298;
    (t4785, t4797, t4798, t4799, t4800, t4804, t4806)
}
