//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2197/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2197<F: Float>(t29547: F, t644: F, t77: F, t1927: F, t5872: F, t2247: F, t5826: F, t27154: F, t98450: F, t28177: F, t7898: F, t28043: F, t4248: F) -> (F, F, F, F, F, F) {
    let t108983 = t77 * t29547 * t644;
    let t108986 = t1927 * t5872;
    let t108990 = t2247 * t5826;
    let t109012 = F::cast_from(6.0_f64) * t98450 * t27154;
    let t109014 = F::cast_from(6.0_f64) * t7898 * t28177;
    let t109024 = F::cast_from(4.0_f64) * t4248 * t28043;
    (t108983, t108986, t108990, t109012, t109014, t109024)
}
