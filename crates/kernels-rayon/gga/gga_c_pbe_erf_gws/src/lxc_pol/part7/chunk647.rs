//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 647/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk647(t1639: f64, t56: f64, t5064: f64, t11: f64, t5048: f64, t625: f64, t25: f64, t5065: f64, t5069: f64, t5072: f64, t5075: f64, t5078: f64, t5082: f64, t5083: f64, t5085: f64, t5087: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5089 = t56 * t1639;
    let t5090 = t5089 * t5064;
    let t5091 = t11 * t5090;
    let t5093 = t625 * t5048;
    let t5094 = t11 * t5093;
    let t5096 = -0.29629629629629629629e-2_f64 * t25 * t5065 + 0.14396666666666666667e0_f64 * t5069 - 0.71983333333333333335e-1_f64 * t5072 - 0.21595e0_f64 * t5075 + 0.21595e0_f64 * t5078 - t5082 - 0.47988888888888888888e-1_f64 * t5083 + 0.35991666666666666666e-1_f64 * t5085 + 0.23994444444444444444e-1_f64 * t5087 - 0.39990740740740740742e-1_f64 * t5091 - 0.35991666666666666667e-1_f64 * t5094;
    (t5089, t5090, t5091, t5093, t5094, t5096)
}
