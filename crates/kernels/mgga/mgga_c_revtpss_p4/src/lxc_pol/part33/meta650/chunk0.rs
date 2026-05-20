//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2101/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2101<F: Float>(t17445: F, t7607: F, t3655: F, t8177: F, t1256: F, t29074: F, t29069: F, t29089: F, t3685: F, t26948: F, t97065: F, t3555: F, t8190: F) -> (F, F, F, F, F, F, F) {
    let t104994 = t7607 * t17445 / F::new(432.0);
    let t104999 = t8177 * t3655;
    let t105002 = F::cast_from(0.57165357490759649296e-3_f64) * t29074 * t1256;
    let t105007 = F::cast_from(0.30488190661738479624e-2_f64) * t29069 * t1256;
    let t105014 = t29089 * t3685 / F::new(162.0);
    let t105046 = t26948 * t97065;
    let t105134 = t3555 * t8190;
    (t104994, t104999, t105002, t105007, t105014, t105046, t105134)
}
