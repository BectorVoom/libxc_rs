//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2616/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2616<F: Float>(t10760: F, t18409: F, t9794: F, t10777: F, t10779: F, t5984: F, t837: F, t18414: F, t40799: F, t18418: F, t18392: F, t236: F, t807: F, t854: F) -> (F, F, F, F, F) {
    let t61981 = t10760 * t9794 * t18409;
    let t61985 = t10777 * t10779 * t5984 * t837;
    let t62012 = t40799 * t9794 * t18414;
    let t62015 = t10760 * t9794 * t18418;
    let t62021 = t807 * t236 * t854 * t18392;
    (t61981, t61985, t62012, t62015, t62021)
}
