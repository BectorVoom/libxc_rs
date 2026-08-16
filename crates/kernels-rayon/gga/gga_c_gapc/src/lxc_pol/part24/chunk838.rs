//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 838/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk838(t2316: f64, t2636: f64, t3378: f64, t1081: f64, t2804: f64, t3375: f64, t9673: f64, t320: f64, t8700: f64, t3379: f64, t3402: f64, t8838: f64) -> (f64, f64, f64, f64, f64) {
    let t10018 = t2636 * t2316;
    let t10019 = t3378 * t10018;
    let t10021 = t1081 * t2804;
    let t10024 = t9673 * t3375;
    let t10026 = t320 * t8700;
    let t10027 = t10026 * t3379;
    let t10029 = t3402 * t8838;
    (t10019, t10021, t10024, t10027, t10029)
}
