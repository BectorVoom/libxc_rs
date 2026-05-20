//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2154/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2154<F: Float>(t1568: F, t7063: F, t25410: F, t25413: F, t27299: F, t689: F, t93281: F, t93317: F, t2439: F, t7774: F, t93170: F, t14489: F, t1579: F, t25286: F, t25292: F, t25317: F, t25383: F, t27199: F, t27317: F, t27322: F, t2771: F, t2828: F, t7053: F, t7070: F, t7071: F, t7759: F, t92870: F, t92873: F, t92875: F) -> (F, F, F) {
    let t98848 = t7063 * t1568;
    let t98849 = t98848 * t25410;
    let t98851 = F::cast_from(0.25702851531048074406e-1_f64) * t98849 * t25413;
    let t98852 = t27299 * t689;
    let t98853 = t93281 * t98852;
    let t98856 = F::cast_from(0.15421710918628844644e0_f64) * t93317 * t98852;
    let t98857 = t7774 * t2439;
    let t98858 = t93170 * t98857;
    let t98864 = F::cast_from(0.17347256376410398924e1_f64) * t25383 * t27317 - t92870 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t25286 * t1579 - t92873 + t92875 + F::cast_from(0.17347256376410398924e1_f64) * t25383 * t27322 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25317 * t7759 * t2771 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t7759 * t2828 + t98851 + F::cast_from(0.86736281882051994623e-1_f64) * t98853 - t98856 - F::cast_from(0.17135234354032049604e-2_f64) * t98858 + F::cast_from(0.17347256376410398924e1_f64) * t27199 * t25292 - F::cast_from(0.39512695097613069591e1_f64) * t7053 * t14489;
    (t98848, t98857, t98864)
}
