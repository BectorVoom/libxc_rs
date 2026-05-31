//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1195/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1195<F: Float>(t102: F, t12930: F, t967: F, t3637: F, t3656: F, t42665: F, t42672: F, t10110: F, t127: F, t34081: F, t34084: F, t34087: F, t42659: F, t42662: F, t42675: F, t42719: F) -> (F, F, F, F, F) {
    let t48760 = F::cast_from(0.233842e2_f64) * t102 * t12930 * t967;
    let t48769 = F::cast_from(0.1052289e3_f64) * t102 * t3656 * t3637;
    let t48771 = F::cast_from(0.116921e2_f64) * t42665;
    let t48772 = F::cast_from(0.19486833333333333333e1_f64) * t42672;
    let t48774 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t34081 + F::cast_from(0.1175232e2_f64) * t34084 - F::cast_from(0.293808e1_f64) * t34087 - F::cast_from(0.3525696e2_f64) * t42659 + t48760 + F::cast_from(0.2350464e2_f64) * t127 * t42719 * t967 - F::cast_from(0.1762848e3_f64) * t127 * t10110 * t3637 - t48769 + F::cast_from(8.0_f64) * t42662 + t48771 + t48772 + F::cast_from(0.293808e1_f64) * t42675;
    (t48760, t48769, t48771, t48772, t48774)
}
