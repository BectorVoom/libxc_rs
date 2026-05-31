//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 690/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk690<F: Float>(t4306: F, t190: F, t4186: F, t706: F, t1531: F, t705: F) -> (F, F, F, F) {
    let t4307 = F::cast_from(4.0_f64) * t4306;
    let t4308 = t190 * t4186;
    let t4310 = F::cast_from(4.0_f64) * t706 * t4308;
    let t4311 = t705 * t1531;
    (t4307, t4308, t4310, t4311)
}
