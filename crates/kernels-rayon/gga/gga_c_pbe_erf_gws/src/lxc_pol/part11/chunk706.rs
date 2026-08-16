//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 706/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk706(t3644: f64, t5825: f64, t3671: f64, t513: f64, t10068: f64, t133: f64, t10071: f64, t10037: f64, t525: f64, t285: f64, t3379: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10110 = t5825 * t3644;
    let t10134 = t3671 * t513;
    let t10168 = t133 * t10068;
    let t10170 = t133 * t10071;
    let t10186 = t525 * t10037;
    let t10207 = t3379 * t545 * t285;
    (t10110, t10134, t10168, t10170, t10186, t10207)
}
