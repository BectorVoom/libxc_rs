//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1179/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1179<F: Float>(t25082: F, t30123: F, t7732: F, t7742: F, t1936: F, t6765: F, t651: F, t18245: F, t1501: F, t1518: F) -> (F, F, F, F, F, F) {
    let t30125 = F::new(6.0) * t25082 * t30123;
    let t30127 = F::new(4.0) * t7732 * t7742;
    let t30128 = t6765 * t1936;
    let t30130 = F::new(2.0) * t651 * t30128;
    let t30137 = F::new(2.0) * t18245 * t1936;
    let t30138 = t1501 * t1518;
    (t30125, t30127, t30128, t30130, t30137, t30138)
}
