//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 971/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk971(t527: f64, t8851: f64, t39846: f64, t542: f64, t138: f64, t1701: f64, t6: f64, t129: f64, t135: f64, t140: f64, t2036: f64, t2037: f64, t2060: f64, t37685: f64, t37894: f64, t37897: f64, t37899: f64, t39849: f64, t39854: f64, t399: f64, t40090: f64, t40123: f64, t543: f64, t555: f64, t7335: f64, t8807: f64, t8838: f64, t8999: f64) -> f64 {
    let t40227 = t527 * t8851;
    let t40234 = t542 * t39846;
    let t40255 = t138 * t6 * t1701;
    let t40258 = -0.35032929183548774392e2_f64 * t40227 * t40090 + 0.4832730710723063824e1_f64 * t8999 * t399 + 0.17516464591774387196e2_f64 * t7335 * t39854 + 0.14498192132169191472e2_f64 * t40234 * t39849 + 0.45910941751869106328e2_f64 * t8838 * t40123 + 0.87582322958871935982e1_f64 * t2036 * t2037 * t2060 + 0.55468804540618892788e2_f64 * t2036 * t8807 * t555 - 0.26248964422271975727e0_f64 * t543 * t37899 + 0.65622411055679939316e-1_f64 * t140 * t37899 - 0.23380572188451859703e3_f64 * t140 * t37685 + 0.19686723316703981795e0_f64 * t129 * t37894 * t37897 * t135 * t40255;
    t40258
}
