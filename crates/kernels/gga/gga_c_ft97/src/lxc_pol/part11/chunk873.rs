//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 873/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk873<F: Float>(t527: F, t8851: F, t39846: F, t542: F, t138: F, t1701: F, t6: F, t129: F, t135: F, t140: F, t2036: F, t2037: F, t2060: F, t37685: F, t37894: F, t37897: F, t37899: F, t39849: F, t39854: F, t399: F, t40090: F, t40123: F, t543: F, t555: F, t7335: F, t8807: F, t8838: F, t8999: F) -> (F,) {
    let t40227 = t527 * t8851;
    let t40234 = t542 * t39846;
    let t40255 = t138 * t6 * t1701;
    let t40258 = -0.35032929183548774392e2 * t40227 * t40090 + 0.4832730710723063824e1 * t8999 * t399 + 0.17516464591774387196e2 * t7335 * t39854 + 0.14498192132169191472e2 * t40234 * t39849 + 0.45910941751869106328e2 * t8838 * t40123 + 0.87582322958871935982e1 * t2036 * t2037 * t2060 + 0.55468804540618892788e2 * t2036 * t8807 * t555 - 0.26248964422271975727e0 * t543 * t37899 + 0.65622411055679939316e-1 * t140 * t37899 - 0.23380572188451859703e3 * t140 * t37685 + 0.19686723316703981795e0 * t129 * t37894 * t37897 * t135 * t40255;
    (t40258,)
}
