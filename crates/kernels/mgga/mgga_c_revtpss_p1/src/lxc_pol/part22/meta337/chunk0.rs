//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1795/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1795<F: Float>(t262: F, t775: F, t3335: F, t389: F, t1077: F, t225: F, t1071: F, t3046: F, t268: F, t271: F, t7021: F) -> (F, F, F, F, F, F, F) {
    let t11088 = t262 * t775;
    let t11108 = F::new(1.0) / t3335 / t389;
    let t11119 = t1077 * t1077;
    let t11120 = F::new(1.0) / t11119;
    let t11121 = t225 * t11120;
    let t11128 = t3046 * t1071;
    let t11132 = t268 * t7021 * t271;
    (t11088, t11108, t11119, t11120, t11121, t11128, t11132)
}
