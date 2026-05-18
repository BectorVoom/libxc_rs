//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 561/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk561<F: Float>(t509: F, t5869: F, t552: F, t557: F, t303: F, t1497: F, t2011: F) -> (F, F, F, F, F) {
    let t5870 = t509 * t5869;
    let t5871 = t5870 * t552;
    let t5872 = t5871 * t557;
    let t5873 = t303 * t5872;
    let t5875 = t2011 * t1497;
    (t5870, t5871, t5872, t5873, t5875)
}
