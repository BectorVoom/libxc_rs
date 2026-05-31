//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 617/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk617<F: Float>(t20: F, t27: F, t12: F, t19: F, t592: F, t596: F, t21: F) -> (F, F, F, F, F) {
    let t2230 = F::cast_from(20.0_f64) * t20 * t27;
    let t2231 = t12 * t19;
    let t2233 = F::cast_from(30.0_f64) * t2231 * t27;
    let t2235 = F::cast_from(72.0_f64) * t592 * t596;
    let t2236 = t21 * t21;
    (t2230, t2231, t2233, t2235, t2236)
}
