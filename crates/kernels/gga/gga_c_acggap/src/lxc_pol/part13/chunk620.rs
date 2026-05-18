//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 620/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk620<F: Float>(t1524: F, t336: F, t429: F, t3114: F, t355: F, t352: F, t1427: F, t721: F, t1049: F, t1483: F, t346: F, t4099: F) -> (F, F, F, F, F, F) {
    let t4791 = t336 * t429 * t1524;
    let t4794 = t3114 * t355;
    let t4795 = t352 * t4794;
    let t4796 = t1427 * t721;
    let t4797 = t4795 * t4796;
    let t4798 = F::new(0.2445e0) * t4797;
    let t4799 = t1049 * t1483;
    let t4800 = F::new(0.978e0) * t4799;
    let t4801 = t346 * t4099;
    (t4791, t4797, t4798, t4799, t4800, t4801)
}
