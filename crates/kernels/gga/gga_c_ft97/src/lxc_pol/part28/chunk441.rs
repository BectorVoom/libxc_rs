//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 441/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk441<F: Float>(t79: F, t355: F, t7205: F, t7204: F, t1291: F, t1295: F, t1303: F, t5587: F, t7173: F, t7178: F, t7181: F, t7183: F, t7191: F, t7196: F, t7202: F) -> (F, F, F) {
    let t80 = 0.1e-59 < t79;
    let t7206 = t7205 * t355;
    let t7207 = t7204 * t7206;
    let t7211 = piecewise3(t80, 2.0 * t7173 - 0.88910709717637694816e-2 * t1295 * t1291 - 0.76612330055555555556e-1 * t7178 * t1303 + 0.22227677429409423704e-2 * t7181 * t7183 + 0.19762785756235085044e-4 * t79 * t7191 + 0.34058283191806748844e-3 * t5587 * t7196 - 0.22227677429409423704e-2 * t79 * t7183 + 0.58694491165413811142e-2 * t7202 * t7207, 0.0);
    (t7206, t7207, t7211)
}
