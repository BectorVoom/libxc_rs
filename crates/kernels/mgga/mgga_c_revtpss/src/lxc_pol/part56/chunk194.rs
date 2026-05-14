//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 194/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk194<F: Float>(t225: F, t679: F, t704: F, t709: F, t718: F, t751: F, t754: F, t759: F, t764: F, t243: F, t73: F, t775: F, t227: F, t229: F) -> (F, F, F, F) {
    let t830 = (t679 + t704 + t709 + t718 + t751 + t754 - t759 - t764) * t225;
    let t832 = t73 * t243;
    let t833 = t832 * t775;
    let t836 = 3.0 * t227 * t833 - t229 * t830;
    (t830, t832, t833, t836)
}
