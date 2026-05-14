//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 541/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk541<F: Float>(t459: F, t5215: F, t1208: F, t1769: F, t487: F, t1770: F, t1214: F, t1774: F) -> (F, F, F, F, F) {
    let t5216 = t5215 * t459;
    let t5219 = t1769 * t1208;
    let t5220 = t5219 * t487;
    let t5225 = t1770 * t487;
    let t5230 = t1774 * t1214;
    (t5216, t5219, t5220, t5225, t5230)
}
