//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2530/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2530<F: Float>(t2439: F, t4625: F, t4622: F, t123: F, t127: F, t159: F) -> (F, F, F, F) {
    let t51913 = t2439 * t4625;
    let t51914 = F::new(0.5519e0) * t51913;
    let t51915 = t2439 * t4622;
    let t51957 = t123 * t127 * t159;
    (t51913, t51914, t51915, t51957)
}
