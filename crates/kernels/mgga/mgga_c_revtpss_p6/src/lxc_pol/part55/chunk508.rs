//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 508/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk508<F: Float>(t1416: F, t3989: F, t1386: F, t240: F, t1398: F, t543: F, t550: F, t2661: F, t1384: F, t544: F) -> (F, F, F, F, F) {
    let t3990 = t3989 * t1416;
    let t3992 = t1386 * t240;
    let t3994 = t550 * t1398 * t543;
    let t3995 = t3992 * t3994;
    let t3996 = t2661 * t3995;
    let t3999 = F::cast_from(1.0_f64) / t1384 / t544;
    (t3990, t3992, t3994, t3996, t3999)
}
