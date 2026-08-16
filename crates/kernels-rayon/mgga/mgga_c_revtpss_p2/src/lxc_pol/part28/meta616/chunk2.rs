//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2154/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2154(t1568: f64, t7063: f64, t25410: f64, t25413: f64, t27299: f64, t689: f64, t93281: f64, t93317: f64, t2439: f64, t7774: f64, t93170: f64, t14489: f64, t1579: f64, t25286: f64, t25292: f64, t25317: f64, t25383: f64, t27199: f64, t27317: f64, t27322: f64, t2771: f64, t2828: f64, t7053: f64, t7070: f64, t7071: f64, t7759: f64, t92870: f64, t92873: f64, t92875: f64) -> (f64, f64, f64) {
    let t98848 = t7063 * t1568;
    let t98849 = t98848 * t25410;
    let t98851 = 0.25702851531048074406e-1_f64 * t98849 * t25413;
    let t98852 = t27299 * t689;
    let t98853 = t93281 * t98852;
    let t98856 = 0.15421710918628844644e0_f64 * t93317 * t98852;
    let t98857 = t7774 * t2439;
    let t98858 = t93170 * t98857;
    let t98864 = 0.17347256376410398924e1_f64 * t25383 * t27317 - t92870 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t25286 * t1579 - t92873 + t92875 + 0.17347256376410398924e1_f64 * t25383 * t27322 - 0.26020884564615598386e1_f64 * t7070 * t25317 * t7759 * t2771 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t7759 * t2828 + t98851 + 0.86736281882051994623e-1_f64 * t98853 - t98856 - 0.17135234354032049604e-2_f64 * t98858 + 0.17347256376410398924e1_f64 * t27199 * t25292 - 0.39512695097613069591e1_f64 * t7053 * t14489;
    (t98848, t98857, t98864)
}
