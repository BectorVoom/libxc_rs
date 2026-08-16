//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1764/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1764<F: Float>(t25240: F, t2710: F, t826: F, t2482: F, t27: F, t7036: F) -> (F, F) {
    let t25242 = t2710 * t25240 * t826;
    let t25243 = F::cast_from(0.90357964994909313586e-5_f64) * t25242;
    let t25245 = t2482 * t7036 * t27;
    (t25243, t25245)
}
