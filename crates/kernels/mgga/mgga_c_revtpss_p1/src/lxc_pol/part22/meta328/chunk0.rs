//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1780/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1780<F: Float>(t2648: F, t2741: F, t2710: F, t826: F, t9732: F, t234: F, t2735: F) -> (F, F, F) {
    let t10881 = t2741 * t2648;
    let t10885 = F::cast_from(0.81322168495418382223e-4_f64) * t2710 * t9732 * t826;
    let t10886 = t2735 * t234;
    (t10881, t10885, t10886)
}
