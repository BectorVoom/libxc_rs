//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1928/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1928<F: Float>(t25207: F, t29598: F, t1468: F, t1544: F, t30: F, t5962: F, t1579: F, t7759: F, t7071: F, t25262: F, t6024: F, t25270: F, t6037: F) -> (F, F, F, F, F, F, F) {
    let t29599 = t25207 * t29598;
    let t29602 = t1468 * t1544;
    let t29606 = t30 * t5962;
    let t29610 = t7759 * t1579;
    let t29611 = t7071 * t29610;
    let t29616 = t25262 * t6024;
    let t29618 = t25270 * t6037;
    (t29599, t29602, t29606, t29610, t29611, t29616, t29618)
}
