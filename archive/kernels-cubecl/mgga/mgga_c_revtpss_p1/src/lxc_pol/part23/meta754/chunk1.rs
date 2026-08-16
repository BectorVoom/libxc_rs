//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2544/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2544<F: Float>(t247: F, t42792: F, t4757: F, t4837: F, t3091: F, t43240: F, t4782: F, t41296: F, t42471: F, t3155: F, t999: F, t1011: F, t4886: F, t697: F) -> (F, F, F, F, F) {
    let t53431 = t4837 * t247 * t42792 * t4757;
    let t53432 = F::cast_from(0.28582678745379824648e-3_f64) * t53431;
    let t53437 = t3091 * t43240 * t4782;
    let t53473 = t42471 * t41296;
    let t53511 = t3155 * t999;
    let t53542 = t1011 * t697 * t4886;
    (t53432, t53437, t53473, t53511, t53542)
}
