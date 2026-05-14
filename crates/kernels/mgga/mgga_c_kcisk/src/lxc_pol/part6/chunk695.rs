//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 695/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk695<F: Float>(t264: F, t259: F, t15613: F, t15626: F, t1121: F, t397: F, t1118: F, t3392: F, t1111: F, t3405: F, t12630: F, t67: F, t10: F, t1102: F, t119: F, t142: F, t260: F, t261: F, t3380: F, t918: F) -> (F, F, F, F, F) {
    let t265 = t264 < -0.66725e-1;
    let t270 = 0.0 < t259;
    let t15627 = t15613 + t15626;
    let t15629 = piecewise3(t270, t15627, -t15627);
    let t15631 = t397 * t1121 * t15629;
    let t15637 = t3392 * t1118;
    let t15643 = t1111 * t3405;
    let t15646 = t67 * t12630;
    let t15660 = piecewise3(t265, 0.0, 10.0 / 9.0 * t260 * t15646 * t10 - 10.0 / 9.0 * t260 * t3380 * t142 + 40.0 / 27.0 * t260 * t1102 * t119 - 280.0 / 243.0 * t260 * t261 * t918);
    (t15627, t15631, t15637, t15643, t15660)
}
