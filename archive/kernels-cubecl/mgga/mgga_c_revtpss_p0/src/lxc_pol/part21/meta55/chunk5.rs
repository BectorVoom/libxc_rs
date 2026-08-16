//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 413/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk413<F: Float>(t265: F, t393: F, t1100: F, t1102: F, t198: F, t336: F, t895: F, t912: F, t938: F, t978: F, t980: F, t985: F) -> F {
    let t394 = t265 < t393;
    let t1106 = piecewise3::<F>(t394, t1100 * t1102 * t198 * t336 - t912 + t938 + t978 + t980 - t985, t895);
    t1106
}
