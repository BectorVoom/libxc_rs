//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3153/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3153<F: Float>(t12772: F, t17639: F, t3625: F, t17645: F, t1284: F, t17288: F, t3624: F, t12917: F, t17401: F, t17396: F, t1260: F, t17289: F) -> (F, F, F, F, F, F) {
    let t57026 = t3625 * t12772 * t17639;
    let t57029 = t3625 * t12772 * t17645;
    let t57040 = t17288 * t1284 * t3624;
    let t57045 = t17401 * t12917;
    let t57049 = t17396 * t12917;
    let t57053 = t17289 * t1260;
    (t57026, t57029, t57040, t57045, t57049, t57053)
}
