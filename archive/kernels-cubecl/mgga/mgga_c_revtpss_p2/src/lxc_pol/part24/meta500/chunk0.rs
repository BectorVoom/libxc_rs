//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1503/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1503<F: Float>(t1558: F, t231: F, t6016: F, t2782: F, t2797: F, t23167: F, t251: F, t2783: F, t76131: F, t18719: F, t51549: F, t23245: F, t2798: F, t686: F, t72: F) -> (F, F, F, F, F, F) {
    let t76161 = t6016 * t1558 * t231;
    let t76163 = t2782 * t2797 * t76161;
    let t76169 = t251 * t23167;
    let t76172 = t2782 * t2783 * t76169 * t231;
    let t76182 = t2782 * t2783 * t76131 * t231;
    let t76206 = t51549 * t18719;
    let t76223 = t2798 * t23245 * t72 * t686;
    (t76163, t76169, t76172, t76182, t76206, t76223)
}
