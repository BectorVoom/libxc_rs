//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 453/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk453<F: Float>(t124: F, t1544: F, t800: F, t1524: F, t1533: F, t1536: F, t225: F, t679: F, t704: F, t751: F, t759: F, t764: F) -> (F, F, F) {
    let t1548 = t124 * t1544;
    let t1549 = t800 * t1548;
    let t1553 = (t679 + t704 + t1524 + t1533 + t751 + t1536 - t759 - t764) * t225;
    (t1548, t1549, t1553)
}
