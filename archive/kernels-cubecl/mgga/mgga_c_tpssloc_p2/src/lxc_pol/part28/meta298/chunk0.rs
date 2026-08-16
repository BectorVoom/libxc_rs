//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1210/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1210<F: Float>(t3069: F, t3180: F, t3036: F, t67: F, t3067: F, t3186: F, t3062: F, t820: F, t3200: F, t3051: F, t3072: F, t3070: F) -> (F, F, F, F, F, F, F) {
    let t10390 = t3180 * t3069;
    let t10401 = t3036 * t67;
    let t10402 = t3067 * t10401;
    let t10403 = t3186 * t10402;
    let t10408 = t820 * t3062;
    let t10413 = t3200 * t10402;
    let t10422 = t820 * t3051;
    let t10423 = t10422 * t3072;
    let t10424 = t3070 * t10423;
    (t10390, t10401, t10403, t10408, t10413, t10422, t10424)
}
