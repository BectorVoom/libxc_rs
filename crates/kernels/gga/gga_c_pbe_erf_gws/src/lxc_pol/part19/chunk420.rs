//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 420/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk420<F: Float>(t732: F, t735: F, t155: F, t266: F, t265: F, t586: F, t615: F) -> (F, F, F, F) {
    let t1615 = t732 * t735;
    let t1617 = t266 * t155;
    let t1619 = F::new(2.0) / F::new(135.0) * t265 * t1617;
    let t1620 = t615 * t586;
    (t1615, t1617, t1619, t1620)
}
