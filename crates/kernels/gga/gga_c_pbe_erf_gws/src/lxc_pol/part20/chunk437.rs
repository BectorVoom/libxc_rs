//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 437/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk437<F: Float>(t1754: F, t1706: F, t187: F, t190: F, t401: F, t607: F, t176: F, t177: F, t191: F) -> (F, F, F, F) {
    let t1844 = 0.47988888888888888889e-1 * t1754;
    let t1851 = 0.11111111111111111111e-1 * t190 * t1706 * t187;
    let t1852 = t401 * t607;
    let t1855 = 1.0 / t177 / t176;
    let t1856 = t191 * t1855;
    (t1844, t1851, t1852, t1856)
}
