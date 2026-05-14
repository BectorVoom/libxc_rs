//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1121/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1121<F: Float>(t94771: F, t97814: F, t27968: F, t3920: F, t25898: F, t98040: F, t25081: F, t7897: F, t198: F, t206: F, t7782: F, t1468: F, t2411: F, t11064: F, t25331: F, t27216: F) -> (F, F, F, F, F, F, F, F) {
    let t98338 = t94771 * t97814;
    let t98372 = t27968 * t3920;
    let t98380 = t98040 * t25898;
    let t98450 = t7897 * t25081;
    let t98637 = t198 * t206 * t7782;
    let t98658 = t2411 * t1468;
    let t98722 = t7782 * t11064;
    let t98825 = t27216 * t25331;
    (t98338, t98372, t98380, t98450, t98637, t98658, t98722, t98825)
}
