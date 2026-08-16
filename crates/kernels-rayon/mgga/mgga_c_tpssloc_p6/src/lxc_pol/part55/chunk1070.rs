//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1070/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1070(t7313: f64, t8875: f64, t2147: f64, t7319: f64, t7327: f64, t7330: f64, t1201: f64, t8878: f64, t1209: f64, t483: f64, t1017: f64, t1207: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32425 = 0.40372756094140390856e-3_f64 * t7313 * t8875;
    let t32428 = t2147 * sigma2;
    let t32429 = t7319 * t32428;
    let t32432 = t7327 * sigma2;
    let t32433 = t32432 * t7330;
    let t32436 = t1201 * t8878;
    let t32439 = t1209 * t483;
    let t32440 = t32439 * t1017;
    let t32441 = t1207 * t32440;
    (t32425, t32428, t32429, t32432, t32433, t32436, t32439, t32440, t32441)
}
