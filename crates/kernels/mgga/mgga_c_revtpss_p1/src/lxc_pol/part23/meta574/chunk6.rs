//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2179/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2179<F: Float>(t10552: F, t10554: F, t23096: F, t23097: F, t23102: F, t23103: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t18556: F) -> (F, F) {
    let t23185 = t23096 - t9278 + t9308 + t9316 + t9329 + t9333 + t23097 - t10552 + t10554 + t23102 + t23103;
    let t23186 = F::cast_from(0.54934341918019635162e-3_f64) * t18556;
    (t23185, t23186)
}
