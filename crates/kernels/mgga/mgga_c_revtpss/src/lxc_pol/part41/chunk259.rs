//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 259/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk259<F: Float>(t225: F, t679: F, t704: F, t709: F, t718: F, t751: F, t754: F, t759: F, t764: F, t243: F, t73: F) -> (F, F) {
    let t830 = (t679 + t704 + t709 + t718 + t751 + t754 - t759 - t764) * t225;
    let t832 = t73 * t243;
    (t830, t832)
}
