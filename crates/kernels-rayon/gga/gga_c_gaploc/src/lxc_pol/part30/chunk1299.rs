//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1299/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1299(t33367: f64, t11050: f64, t11090: f64, t2049: f64, t28585: f64, t32314: f64, t33325: f64, t33328: f64, t33335: f64, t33338: f64, t33351: f64, t33353: f64, t33356: f64, t33359: f64, t33363: f64, t33365: f64, t3496: f64, t531: f64, t5669: f64, t6021: f64, t797: f64) -> f64 {
    let t33368 = 0.89376224879626066674e-1_f64 * t33367;
    let t33369 = -t33325 + t33328 - t33335 - t33338 + 0.1022478025437886658e1_f64 * t5669 * t11050 - 0.35750489951850426669e0_f64 * t797 * t531 * t32314 - 0.23005755572352449806e1_f64 * t6021 * t3496 - 0.71500979903700853338e0_f64 * t2049 * t11090 - t28585 - t33351 - t33353 + t33356 - t33359 - t33363 - t33365 - t33368;
    t33369
}
