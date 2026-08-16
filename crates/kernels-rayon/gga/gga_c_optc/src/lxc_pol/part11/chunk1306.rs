//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1306/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1306(t10409: f64, t1366: f64, t16686: f64, t16817: f64, t2530: f64, t2537: f64, t4904: f64, t4919: f64, t4923: f64, t50490: f64, t56950: f64, t56952: f64, t56954: f64, t56957: f64, t57113: f64, t57117: f64, t57215: f64, t57219: f64, t57222: f64, t57225: f64, t57228: f64, t57233: f64, t7813: f64) -> f64 {
    let t57275 = -t56950 - t56952 - t56954 - t56957 - t57113 - t57117 - t57215 + 0.21053604230838734656e2_f64 * t2537 * t4904 * t4919 - t57219 + t57222 - t57225 - t57228 + t57233 + 0.2077890707925103596e3_f64 * t10409 * t16686 - 0.62336721237753107879e3_f64 * t7813 * t4923 * t4919 - 0.46785787179641632568e1_f64 * t2530 * t16817 * t1366 + 0.69263023597503453196e2_f64 * t2537 * t50490 * t1366;
    t57275
}
