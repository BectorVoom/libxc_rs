//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1051/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1051<F: Float>(t102: F, t12930: F, t967: F, t3637: F, t3656: F, t42665: F, t42672: F, t10110: F, t127: F, t34081: F, t34084: F, t34087: F, t42659: F, t42662: F, t42675: F, t42719: F) -> (F, F, F, F, F) {
    let t48760 = 0.233842e2 * t102 * t12930 * t967;
    let t48769 = 0.1052289e3 * t102 * t3656 * t3637;
    let t48771 = 0.116921e2 * t42665;
    let t48772 = 0.19486833333333333333e1 * t42672;
    let t48774 = -4.0 / 3.0 * t34081 + 0.1175232e2 * t34084 - 0.293808e1 * t34087 - 0.3525696e2 * t42659 + t48760 + 0.2350464e2 * t127 * t42719 * t967 - 0.1762848e3 * t127 * t10110 * t3637 - t48769 + 8.0 * t42662 + t48771 + t48772 + 0.293808e1 * t42675;
    (t48760, t48769, t48771, t48772, t48774)
}
