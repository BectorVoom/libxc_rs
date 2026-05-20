//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2244/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2244<F: Float>(t16423: F, t16475: F, t16526: F, t16589: F, t1079: F, t1071: F, t4746: F) -> (F, F, F) {
    let t16591 = t16423 + t16475 + t16526 + t16589;
    let t16592 = t1079 * t16591;
    let t16597 = t4746 * t1071;
    (t16591, t16592, t16597)
}
