//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2117/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2117<F: Float>(t25188: F, t7935: F, t2014: F, t25802: F, t7934: F, t28167: F, t35070: F, t5627: F, t25081: F, t7897: F, t25083: F, t28020: F, t7315: F) -> (F, F, F, F, F) {
    let t98440 = t25188 * t7935;
    let t98442 = t2014 * t7934 * t25802;
    let t98449 = F::new(12.0) * t28167 * t35070 * t5627;
    let t98450 = t7897 * t25081;
    let t98452 = F::new(6.0) * t98450 * t25083;
    let t98455 = F::new(2.0) * t2014 * t28020 * t7315;
    (t98440, t98442, t98449, t98452, t98455)
}
