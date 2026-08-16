//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2785/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2785<F: Float>(t3917: F, t74835: F, t14090: F, t14100: F, t22432: F, t47603: F, t686: F, t72: F, t22427: F, t2435: F, t1358: F, t212: F, t22307: F, t689: F) -> (F, F, F, F, F) {
    let t74836 = t74835 * t3917;
    let t74838 = t14100 * t14090;
    let t74843 = t47603 * t22432 * t72 * t686;
    let t74849 = t2435 * t22427;
    let t74853 = t689 * t212 * t22307 * t1358;
    (t74836, t74838, t74843, t74849, t74853)
}
