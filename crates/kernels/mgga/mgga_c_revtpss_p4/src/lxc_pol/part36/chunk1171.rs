//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1171/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1171<F: Float>(t225: F, t29636: F, t1949: F, t6048: F, t25317: F, t6071: F, t7071: F, t233: F, t1957: F, t1558: F, t231: F, t7759: F) -> (F, F, F, F, F, F, F, F) {
    let t29637 = t29636 * t225;
    let t29643 = t1949 * t6048;
    let t29644 = t25317 * t29643;
    let t29654 = t1949 * t6071;
    let t29655 = t7071 * t29654;
    let t29658 = t233 * t29636;
    let t29659 = t1957 * t29658;
    let t29668 = t7759 * t1558 * t231;
    (t29637, t29643, t29644, t29654, t29655, t29658, t29659, t29668)
}
