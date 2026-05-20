//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1588/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1588<F: Float>(t43854: F, t43881: F, t43883: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F) -> F {
    let t43907 = t43881 - F::new(8.0) * t43854 + F::new(16.0) / F::new(9.0) * t43883 + F::new(40.0) / F::new(9.0) * t43886 - F::new(112.0) / F::new(81.0) * t43888 + F::new(8.0) / F::new(9.0) * t43890 + F::new(16.0) / F::new(9.0) * t43892 - F::new(8.0) / F::new(3.0) * t43894 - F::new(4.0) / F::new(9.0) * t43896 - F::new(8.0) * t43899 + F::new(8.0) * t43902 + t43905 / F::new(3.0);
    t43907
}
