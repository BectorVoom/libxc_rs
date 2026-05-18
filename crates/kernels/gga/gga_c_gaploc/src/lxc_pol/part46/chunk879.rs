//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 879/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk879<F: Float>(t204: F, t41749: F, t587: F, t41738: F, t6710: F, t6711: F, t6717: F, t6914: F, t12943: F, t4379: F, t40449: F, t40452: F) -> (F, F, F, F, F, F) {
    let t42309 = F::new(0.18404604457881959845e2) * t587 * t204 * t41749;
    let t42312 = F::new(0.43710935587469654631e2) * t6710 * t6711 * t41738;
    let t42315 = F::new(0.12423108009070322895e3) * t6914 * t6717 * t41749;
    let t42316 = t4379 * t12943;
    let t42340 = F::new(0.63904876589867916127e-1) * t40449;
    let t42341 = F::new(0.31952438294933958063e0) * t40452;
    (t42309, t42312, t42315, t42316, t42340, t42341)
}
