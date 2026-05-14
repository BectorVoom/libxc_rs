//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 828/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk828<F: Float>(t102: F, t3660: F, t481: F, t10102: F, t10106: F, t10107: F, t10110: F, t10117: F, t10123: F, t10126: F, t127: F, t2873: F, t2893: F, t5836: F, t8200: F, t10: F, t10089: F, t10090: F, t10094: F, t10096: F, t10097: F, t496: F, t5784: F, t5810: F, t8148: F, t8149: F, t8158: F, t8160: F) -> (F, F) {
    let t10129 = 0.584605e1 * t102 * t3660 * t481;
    let t10130 = -t10102 - 4.0 / 9.0 * t8200 + t5836 - t10106 - 0.146904e1 * t127 * t10107 - 0.293808e2 * t127 * t10110 * t481 + 0.1175232e2 * t127 * t2893 * t2873 + 0.587616e1 * t127 * t10117 * t481 - t10123 + t10126 + t10129;
    let t10132 = t10089 + t10090 + t8148 - 0.195872e1 * t8149 + t8158 - 0.97936e0 * t8160 - 2.0 / 9.0 * t5784 + t10094 - 0.97935999999999999999e0 * t5810 - t10096 + 3.0 / 2.0 * t496 * t10 * t10097 + t10130;
    (t10129, t10132)
}
