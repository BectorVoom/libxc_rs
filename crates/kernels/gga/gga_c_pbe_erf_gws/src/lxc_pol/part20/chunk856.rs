//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 856/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk856<F: Float>(t10513: F, t625: F, t11: F, t10561: F, t10564: F, t10567: F, t10570: F, t10573: F, t10576: F, t10579: F, t10581: F, t10583: F, t10585: F, t5047: F, t5082: F, t7279: F, t7280: F, t7288: F, t7290: F) -> (F, F) {
    let t10587 = t625 * t10513;
    let t10588 = t11 * t10587;
    let t10591 = -0.39990740740740740742e-1 * t10561 + 0.14396666666666666667e0 * t10564 + 0.9597777777777777778e-1 * t10567 - 0.21595e0 * t10570 - 0.28793333333333333334e0 * t10573 - 0.23994444444444444445e-1 * t10576 + 0.71983333333333333334e-1 * t10579 - t5047 - t5082 + 0.79981481481481481483e-2 * t10581 - 0.23994444444444444445e-1 * t10583 + 0.11997222222222222222e-1 * t10585 - 0.35991666666666666667e-1 * t10588 + t7279 - 0.47988888888888888888e-1 * t7280 - t7288 + t7290;
    (t10588, t10591)
}
