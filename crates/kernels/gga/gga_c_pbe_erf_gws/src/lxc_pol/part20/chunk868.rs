//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 868/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk868<F: Float>(t3440: F, t401: F, t3434: F, t3437: F, t572: F, t9788: F, t606: F, t10438: F, t10443: F, t1856: F, t3342: F, t4957: F, t418: F, t25: F, t4941: F, t5241: F, t5256: F, t5271: F, t7335: F, t7364: F, t7374: F, t7376: F, t7379: F, t7380: F) -> (F, F, F) {
    let t10756 = t401 * t3440;
    let t10758 = t401 * t3434;
    let t10760 = t401 * t3437;
    let t10762 = t572 * t9788;
    let t10763 = t606 * t10762;
    let t10771 = t606 * t10438;
    let t10774 = t1856 * t10443;
    let t10777 = t4957 * t3342;
    let t10778 = t10777 * t418;
    let t10779 = t1856 * t10778;
    let t10782 = 0.44444444444444444445e-2 * t10756 + 0.14814814814814814815e-2 * t10758 - 0.88888888888888888887e-2 * t10760 - 0.66666666666666666667e-2 * t25 * t10763 - 0.15996296296296296296e-1 * t4941 - t5241 + t7335 - t7364 - t5271 - 0.31992592592592592592e-1 * t7374 + 0.47988888888888888888e-1 * t7376 + t7379 - 0.47988888888888888888e-1 * t7380 - 0.74074074074074074073e-2 * t5256 + 0.13333333333333333333e-1 * t25 * t10771 - 0.22222222222222222222e-2 * t25 * t10774 + 0.13333333333333333333e-1 * t25 * t10779;
    (t10762, t10778, t10782)
}
