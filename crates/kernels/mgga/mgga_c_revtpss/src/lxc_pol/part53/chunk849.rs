//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 849/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk849<F: Float>(t1950: F, t2453: F, t2458: F, t25372: F, t25410: F, t25413: F, t2411: F, t7086: F) -> (F, F, F, F) {
    let t25422 = t2453 * t1950;
    let t25424 = F::new(0.11565819519348392139e-2) * t25422 * t2458;
    let t25431 = t25372 * t25410;
    let t25432 = t25431 * t25413;
    let t25440 = t7086 * t2411;
    (t25424, t25431, t25432, t25440)
}
