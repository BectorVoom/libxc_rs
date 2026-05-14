//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1069/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1069<F: Float>(t1009: F, t14395: F, t2178: F, t3217: F, t1014: F, t27971: F, t27974: F, t7687: F, t27940: F, t3245: F, t8051: F, t15573: F, t2173: F, t27918: F, t26717: F, t27895: F) -> (F, F, F, F, F, F, F, F) {
    let t96241 = t14395 * t1009;
    let t96249 = t2178 * t3217;
    let t96261 = t1014 * t27971;
    let t96264 = 0.46336805555555555556e-3 * t7687 * t27974;
    let t96270 = t1014 * t27940;
    let t96273 = t3245 * t8051;
    let t96281 = 0.46336805555555555556e-3 * t2173 * t15573 * t27918;
    let t96302 = 0.61836467013888888889e-4 * t27895 * t26717;
    (t96241, t96249, t96261, t96264, t96270, t96273, t96281, t96302)
}
