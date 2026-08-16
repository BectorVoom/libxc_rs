//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1747/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1747<F: Float>(t3133: F, t4982: F, t12131: F, t1071: F, t1089: F, t999: F, t3046: F, t3286: F) -> (F, F, F, F, F) {
    let t12132 = t4982 * t3133;
    let t12133 = t12131 * t12132;
    let t12137 = t1071 * t3133 * t1089;
    let t12143 = t999 * t3133 * t1089;
    let t12146 = t3046 * t3286;
    (t12132, t12133, t12137, t12143, t12146)
}
