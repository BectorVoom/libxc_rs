//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2969/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2969<F: Float>(t13850: F, t2482: F, t2668: F, t4000: F, t13841: F, t9962: F, t4010: F, t808: F, t13785: F, t48862: F, t13817: F, t13999: F) -> (F, F, F, F, F) {
    let t48982 = t2482 * t4000 * t2668 * t13850;
    let t48984 = t9962 * t13841;
    let t48999 = t808 * t4010;
    let t49001 = t48862 * t48999 * t13785;
    let t49003 = t13999 * t13817;
    (t48982, t48984, t48999, t49001, t49003)
}
