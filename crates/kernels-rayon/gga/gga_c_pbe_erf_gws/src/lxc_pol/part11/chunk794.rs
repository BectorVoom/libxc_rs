//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 794/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk794(t12493: f64, t5061: f64, t12513: f64, t657: f64, t10517: f64, t12499: f64, t12503: f64, t12507: f64, t12511: f64, t25: f64, t5047: f64, t5082: f64, t7239: f64, t7269: f64) -> (f64, f64, f64) {
    let t12855 = t5061 * t12493;
    let t12858 = t657 * t12513;
    let t12868 = -t5047 - 0.29629629629629629629e-2_f64 * t25 * t12855 - 0.66666666666666666667e-2_f64 * t25 * t12858 + 0.44444444444444444445e-2_f64 * t10517 + 0.14396666666666666667e0_f64 * t12499 - 0.71983333333333333335e-1_f64 * t12503 - 0.21595e0_f64 * t12507 + 0.21595e0_f64 * t12511 - 0.22222222222222222222e-1_f64 * t7239 - t5082 - 0.47988888888888888888e-1_f64 * t7269;
    (t12855, t12858, t12868)
}
