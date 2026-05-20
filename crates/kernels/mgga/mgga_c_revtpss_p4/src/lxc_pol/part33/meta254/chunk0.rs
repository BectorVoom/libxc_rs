//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1132/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1132<F: Float>(t624: F, t640: F, t76: F, t1937: F, t2322: F, t4254: F, t1310: F, t1936: F) -> (F, F, F, F, F) {
    let t6971 = F::new(8.0) / F::new(3.0) * t624;
    let t6977 = t76 * t640;
    let t6990 = F::new(2.0) * t2322 * t1937;
    let t6992 = F::new(2.0) * t4254 * t1937;
    let t6993 = t1310 * t1936;
    (t6971, t6977, t6990, t6992, t6993)
}
