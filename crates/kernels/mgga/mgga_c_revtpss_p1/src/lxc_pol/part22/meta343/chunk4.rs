//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1822/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1822<F: Float>(t11132: F, t11337: F, t3010: F, t963: F) -> (F, F, F) {
    let t11479 = F::cast_from(0.93932222222222222223e0_f64) * t11132;
    let t11480 = F::cast_from(0.36793333333333333333e0_f64) * t11337;
    let t11506 = F::new(1.0) / t3010 / t963;
    (t11479, t11480, t11506)
}
