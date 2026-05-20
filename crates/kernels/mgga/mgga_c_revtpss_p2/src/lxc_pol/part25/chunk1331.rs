//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1331/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1331<F: Float>(t1426: F, t7275: F, t786: F, t3917: F, t1444: F, t3923: F, t543: F, t25953: F, t26072: F, t2435: F, t25913: F, t7289: F, t94600: F) -> (F, F, F, F, F) {
    let t94748 = t786 * t7275 * t1426;
    let t94749 = t94748 * t3917;
    let t94752 = t1444 * t3923 * t543;
    let t94756 = t26072 * t25953;
    let t94758 = t2435 * t25913;
    let t94761 = F::cast_from(0.39982213492741449076e-1_f64) * t7289 * t94600;
    (t94749, t94752, t94756, t94758, t94761)
}
