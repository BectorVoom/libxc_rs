//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2635/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2635<F: Float>(t1225: F, t18281: F, t1012: F, t1010: F, t5843: F, t5378: F, t5381: F, t21040: F, t3629: F, t3626: F, t12840: F, t20795: F) -> (F, F, F, F, F, F, F) {
    let t21209 = t1225 * t18281;
    let t21210 = t1012 * t21209;
    let t21213 = t5843 * t1010;
    let t21216 = t5381 * t5378;
    let t21218 = t21040 * t3629;
    let t21219 = t3626 * t21218;
    let t21222 = t20795 * t12840;
    (t21209, t21210, t21213, t21216, t21218, t21219, t21222)
}
