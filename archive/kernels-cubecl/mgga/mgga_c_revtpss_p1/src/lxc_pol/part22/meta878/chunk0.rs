//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3045/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3045<F: Float>(t14575: F, t2435: F, t10943: F, t14598: F, t686: F, t72: F, t10541: F, t14495: F, t2782: F, t10518: F, t14568: F, t1568: F, t4503: F) -> (F, F, F, F, F) {
    let t51537 = t2435 * t14575;
    let t51541 = t14598 * t10943 * t72 * t686;
    let t51544 = t2782 * t10541 * t14495;
    let t51546 = t14568 * t10518;
    let t51548 = t4503 * t1568;
    (t51537, t51541, t51544, t51546, t51548)
}
