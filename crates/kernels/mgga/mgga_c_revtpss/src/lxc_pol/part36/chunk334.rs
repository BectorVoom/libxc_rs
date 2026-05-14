//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 334/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk334<F: Float>(t1524: F, t1533: F, t1536: F, t225: F, t679: F, t704: F, t751: F, t759: F, t764: F, t1544: F, t832: F, t227: F, t229: F) -> (F, F, F) {
    let t1553 = (t679 + t704 + t1524 + t1533 + t751 + t1536 - t759 - t764) * t225;
    let t1555 = t832 * t1544;
    let t1558 = -t1553 * t229 + 3.0 * t1555 * t227;
    (t1553, t1555, t1558)
}
