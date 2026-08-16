//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 472/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk472<F: Float>(t1626: F, t324: F, t1594: F, t1601: F, t1604: F, t1607: F, t967: F, t970: F) -> (F, F) {
    let t1627 = t1626 * t324;
    let t1633 = F::cast_from(0.258925e1_f64) * t1601 - t967 - F::cast_from(0.301925e0_f64) * t1594 + F::cast_from(0.16504875e0_f64) * t1604 - t970 - F::cast_from(0.82785e-1_f64) * t1607;
    (t1627, t1633)
}
