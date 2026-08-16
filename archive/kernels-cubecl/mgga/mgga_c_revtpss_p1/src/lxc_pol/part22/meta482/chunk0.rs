//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2195/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2195<F: Float>(t11710: F, t4782: F, t3091: F, t1014: F, t140: F) -> (F, F, F) {
    let t15984 = t11710 * t4782;
    let t15986 = F::cast_from(0.19055119163586549765e-3_f64) * t3091 * t15984;
    let t15987 = t140 * t1014;
    (t15984, t15986, t15987)
}
