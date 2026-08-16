//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1293/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1293<F: Float>(t30799: F, t800: F, t21270: F, t2137: F, t467: F, t20926: F, t26870: F, t20850: F, t2138: F, t29086: F, t5362: F, t21169: F, t7607: F) -> (F, F, F, F, F, F) {
    let t112350 = t30799 * t800;
    let t112356 = t467 * t2137 * t21270;
    let t112364 = t26870 * t20926;
    let t112373 = t20850 * t2138;
    let t112380 = t29086 * t5362;
    let t112397 = t7607 * t21169;
    (t112350, t112356, t112364, t112373, t112380, t112397)
}
