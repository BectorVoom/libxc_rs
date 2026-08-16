//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1841/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1841<F: Float>(t1416: F, t94545: F, t240: F, t25981: F, t25987: F, t9775: F, t2453: F, t4086: F, t64: F, t9795: F, t2018: F, t40688: F, t46808: F) -> (F, F, F, F, F, F) {
    let t94546 = t94545 * t1416;
    let t94550 = t25981 * t240;
    let t94554 = t9775 * t25987;
    let t94564 = t2453 * t4086 * t64;
    let t94565 = t94564 * t9795;
    let t94568 = t40688 * t2018 * t46808;
    (t94546, t94550, t94554, t94564, t94565, t94568)
}
