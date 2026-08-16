//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1730/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1730<F: Float>(t3940: F, t9962: F, t221: F, t3924: F, t4019: F, t4018: F, t3930: F, t4059: F, t1386: F, t2482: F, t596: F) -> (F, F, F, F, F) {
    let t9963 = t9962 * t3940;
    let t9970 = t4019 * t221 * t3924;
    let t9971 = t4018 * t9970;
    let t9973 = t3930 * t4059;
    let t9976 = t2482 * t1386 * t596;
    (t9963, t9970, t9971, t9973, t9976)
}
