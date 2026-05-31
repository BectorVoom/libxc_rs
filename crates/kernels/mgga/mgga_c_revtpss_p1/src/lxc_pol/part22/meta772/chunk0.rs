//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2858/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2858<F: Float>(t1222: F, t3693: F, t697: F, t12256: F, t3698: F, t3362: F, t414: F, t3551: F, t3565: F, t225: F, t480: F, t12884: F, t828: F) -> (F, F, F, F, F, F, F) {
    let t44343 = t1222 * t697 * t3693;
    let t44348 = t3698 * t12256;
    let t44361 = F::cast_from(1.0_f64) / t414 / t3362;
    let t44420 = t3551 * t3565;
    let t44421 = t44420 * t225;
    let t44422 = t44421 * t480;
    let t44425 = t828 * t12884;
    (t44343, t44348, t44361, t44420, t44421, t44422, t44425)
}
