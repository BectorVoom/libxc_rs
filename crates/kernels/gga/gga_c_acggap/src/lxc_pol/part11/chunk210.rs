//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 210/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk210<F: Float>(t4: F, t668: F, t11: F, t19: F, t662: F, t210: F, t665: F, t21: F, t351: F, t5: F) -> (F, F, F, F, F) {
    let t669 = t4 * t668;
    let t671 = F::new(1.0)/f64::sqrt(t11);
    let t672 = t671 * t19;
    let t673 = t672 * t662;
    let t675 = t210 * t665;
    let t678 = t21 * t5 * t351;
    (t669, t672, t673, t675, t678)
}
