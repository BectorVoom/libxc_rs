//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1255/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1255<F: Float>(t2453: F, t2458: F, t7049: F, t1950: F, t2769: F, t786: F, t10997: F, t231: F, t2645: F, t886: F, t25404: F, t40270: F) -> (F, F, F, F) {
    let t93252 = t2453 * t7049 * t2458;
    let t93261 = t786 * t1950 * t2769;
    let t93262 = t93261 * t10997;
    let t93267 = t886 * t2645 * t231;
    let t93272 = F::new(0.96373646535613327356e-3) * t40270 * t25404;
    (t93252, t93262, t93267, t93272)
}
