//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1100/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1100<F: Float>(t26379: F, t26702: F, t3: F, t2055: F, t2327: F, t116: F, t7373: F, t670: F, t2371: F, t7553: F, t117: F, t26153: F) -> (F, F, F, F, F, F, F, F) {
    let t26703 = t26379 + t26702;
    let t26704 = t3 * t26703;
    let t26716 = param_d * t26703;
    let t26730 = t2327 * t2055;
    let t26733 = t116 * t7373;
    let t26734 = t26733 * t670;
    let t26737 = t7553 * t2371;
    let t26740 = t117 * t26153;
    (t26703, t26704, t26716, t26730, t26733, t26734, t26737, t26740)
}
