//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1057/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1057<F: Float>(t117: F, t32608: F, t1310: F, t2322: F, t32402: F, t32404: F, t32410: F, t32415: F, t32417: F, t32419: F, t32421: F, t32576: F, t32580: F, t4254: F, t508: F, t651: F, t6985: F, t7378: F, t8627: F, t8637: F) -> (F, F) {
    let t32609 = t32608 * t117;
    let t32612 = -t1310 * t8627 - F::new(2.0) * t2322 * t8637 - F::new(2.0) * t32410 * t651 - t32609 * t508 - F::new(2.0) * t4254 * t8637 - F::new(2.0) * t6985 * t7378 - F::new(2.0) * t32402 - F::new(2.0) * t32404 - t32415 - t32417 - t32419 - t32421 - t32576 + t32580;
    (t32609, t32612)
}
