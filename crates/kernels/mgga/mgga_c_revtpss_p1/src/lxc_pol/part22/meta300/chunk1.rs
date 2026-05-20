//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1732/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1732<F: Float>(t2482: F, t27: F, t4000: F, t221: F, t4004: F, t4019: F, t1419: F, t4086: F, t786: F) -> (F, F, F, F, F) {
    let t10001 = t2482 * t4000 * t27;
    let t10003 = t4019 * t221 * t4004;
    let t10004 = t10001 * t10003;
    let t10013 = t4086 * t1419;
    let t10014 = t786 * t10013;
    (t10001, t10003, t10004, t10013, t10014)
}
