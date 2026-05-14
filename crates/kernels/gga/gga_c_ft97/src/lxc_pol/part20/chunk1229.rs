//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1229/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1229<F: Float>(t1882: F, t28793: F, t28535: F, t7080: F, t8232: F, t18: F, t25044: F, t2665: F, t3281: F, t10683: F, t28501: F, t446: F, t824: F, t112376: F, t1486: F, t193: F, t852: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113249 = t1882 * t28793;
    let t113250 = 4.0 * t113249;
    let t113251 = t1882 * t28535;
    let t113252 = 2.0 / 9.0 * t113251;
    let t113253 = t8232 * t7080;
    let t113254 = 4.0 / 27.0 * t113253;
    let t113257 = t3281 * t2665 * t25044 * t18;
    let t113261 = t446 * t10683 * t28501 * t824;
    let t113265 = t1486 * t193 * t852 * t112376;
    (t113249, t113250, t113251, t113252, t113253, t113254, t113257, t113261, t113265)
}
