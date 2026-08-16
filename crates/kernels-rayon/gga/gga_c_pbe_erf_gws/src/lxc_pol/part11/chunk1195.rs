//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1195/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1195(t102: f64, t12930: f64, t967: f64, t3637: f64, t3656: f64, t42665: f64, t42672: f64, t10110: f64, t127: f64, t34081: f64, t34084: f64, t34087: f64, t42659: f64, t42662: f64, t42675: f64, t42719: f64) -> (f64, f64, f64, f64, f64) {
    let t48760 = 0.233842e2_f64 * t102 * t12930 * t967;
    let t48769 = 0.1052289e3_f64 * t102 * t3656 * t3637;
    let t48771 = 0.116921e2_f64 * t42665;
    let t48772 = 0.19486833333333333333e1_f64 * t42672;
    let t48774 = -4.0_f64 / 3.0_f64 * t34081 + 0.1175232e2_f64 * t34084 - 0.293808e1_f64 * t34087 - 0.3525696e2_f64 * t42659 + t48760 + 0.2350464e2_f64 * t127 * t42719 * t967 - 0.1762848e3_f64 * t127 * t10110 * t3637 - t48769 + 8.0_f64 * t42662 + t48771 + t48772 + 0.293808e1_f64 * t42675;
    (t48760, t48769, t48771, t48772, t48774)
}
