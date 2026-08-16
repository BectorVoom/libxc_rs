//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1002/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1002<F: Float>(t12898: F, t481: F, t3172: F, t3605: F, t3600: F, t11262: F, t1251: F, t1247: F, t3704: F, t3708: F, t1284: F, t3566: F) -> (F, F, F, F, F) {
    let t12900 = F::cast_from(0.63517063878621832551e-4_f64) * t481 * t12898;
    let t12901 = t3172 * t3605;
    let t12902 = t3600 * t12901;
    let t12904 = t11262 * t1251;
    let t12905 = t1247 * t12904;
    let t12907 = t3708 * t3704;
    let t12909 = t3566 * t1284;
    (t12900, t12902, t12905, t12907, t12909)
}
