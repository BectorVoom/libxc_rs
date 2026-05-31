//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 527/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk527<F: Float>(t2289: F, t45: F, t631: F, t78: F, t57: F, t635: F, t81: F, t116: F, t648: F) -> (F, F, F, F, F, F) {
    let t2290 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2289;
    let t2297 = t631 * t45;
    let t2299 = F::cast_from(1.0_f64) / t78 / t2297;
    let t2304 = t635 * t57;
    let t2306 = F::cast_from(1.0_f64) / t81 / t2304;
    let t2322 = t648 * t116;
    (t2290, t2297, t2299, t2304, t2306, t2322)
}
