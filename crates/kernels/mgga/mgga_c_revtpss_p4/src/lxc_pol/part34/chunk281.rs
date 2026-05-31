//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 281/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk281<F: Float>(t1118: F, t458: F, t456: F) -> (F, F, F, F) {
    let t1201 = F::cast_from(0.83333333333333333333e-2_f64) * t1118;
    let t1207 = t458 * t458;
    let t1208 = F::cast_from(1.0_f64) / t1207;
    let t1209 = t456 * t1208;
    (t1201, t1207, t1208, t1209)
}
