//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 172/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk172<F: Float>(t213: F, t218: F, t607: F, t64: F, t215: F, t220: F, t43: F, t130: F, t139: F, t145: F, t459: F, t464: F, t458: F, t129: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t847 = -t64 - t607;
    let t850 = piecewise3(t214, 0.0, 4.0 / 3.0 * t215 * t847);
    let t851 = -t847;
    let t854 = piecewise3(t219, 0.0, 4.0 / 3.0 * t220 * t851);
    let t856 = (t850 + t854) * t43;
    let t860 = t130 * t139;
    let t862 = t860 * t145 * t459;
    let t864 = t464 * t130;
    let t866 = t139 * t145 * t458;
    let t867 = t864 * t866;
    let t869 = 3.0 / 128.0 * t862 - t867 / 128.0;
    let t871 = 1.0 / t129;
    (t856, t860, t862, t864, t866, t867, t869, t871)
}
