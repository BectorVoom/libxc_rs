//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1695/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1695<F: Float>(t1337: F, t9586: F, t4146: F, t565: F, t1333: F, t3860: F, t4144: F, t4147: F, t30: F, t513: F, t33: F, t516: F) -> (F, F, F, F, F, F, F, F) {
    let t9588 = F::cast_from(0.56968947174242584612e-3_f64) * t1337 * t9586;
    let t9593 = F::cast_from(1.0_f64) / t4146 / t565;
    let t9597 = t3860 * t1333;
    let t9599 = t4144 * t4147;
    let t9603 = t30 * t30;
    let t9605 = F::cast_from(1.0_f64) / t513 / t9603;
    let t9615 = t33 * t33;
    let t9617 = F::cast_from(1.0_f64) / t516 / t9615;
    (t9588, t9593, t9597, t9599, t9603, t9605, t9615, t9617)
}
