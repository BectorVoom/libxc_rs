//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 671/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk671(t185: f64, t5357: f64, t5081: f64, t5042: f64, t5069: f64, t5072: f64, t5075: f64, t5078: f64, t5083: f64, t5085: f64, t5087: f64, t5091: f64, t5094: f64) -> (f64, f64) {
    let t5359 = 16.0_f64 / 405.0_f64 * t185 * t5357;
    let t5360 = 0.58774074074074074074e-2_f64 * t5081;
    let t5371 = t5360 + 0.25188888888888888889e-2_f64 * t5083 - 0.12594444444444444445e-2_f64 * t5087 + 0.37783333333333333335e-2_f64 * t5042 - 0.18891666666666666667e-2_f64 * t5085 + 0.20990740740740740742e-2_f64 * t5091 - 0.75566666666666666669e-2_f64 * t5069 + 0.37783333333333333335e-2_f64 * t5072 + 0.11335e-1_f64 * t5075 - 0.11335e-1_f64 * t5078 + 0.18891666666666666667e-2_f64 * t5094;
    (t5359, t5371)
}
