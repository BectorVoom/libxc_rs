//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 180/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk180<F: Float>(t20: F, t588: F, t12: F, t19: F, t2: F, t27: F, t21: F, t579: F) -> (F, F, F, F, F) {
    let t590 = F::cast_from(4.0_f64) * t20 * t588;
    let t592 = t12 * t19 * t2;
    let t594 = F::cast_from(6.0_f64) * t592 * t27;
    let t595 = t21 * t579;
    let t596 = F::cast_from(1.0_f64) / t595;
    (t590, t592, t594, t595, t596)
}
