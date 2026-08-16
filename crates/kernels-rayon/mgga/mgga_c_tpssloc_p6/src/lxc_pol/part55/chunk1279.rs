//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1279/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1279(t24721: f64, t7337: f64, t8039: f64, t118002: f64, t118005: f64, t118007: f64, t119243: f64, t125453: f64, t2134: f64, t27580: f64, t27654: f64, t27704: f64, t27714: f64, t32428: f64, t32429: f64, t34260: f64, t4973: f64, t7316: f64, t8031: f64, t8875: f64) -> f64 {
    let t125474 = t24721 * t7337 * t8039;
    let t125482 = 0.40372756094140390856e-3_f64 * t8031 * t32429 - 0.40372756094140390856e-3_f64 * t2134 * t27654 * t32428 + 0.40372756094140390856e-3_f64 * t7316 * t34260 - t125453 * t119243 * t4973 / 1152.0_f64 + 0.32298204875312312685e-2_f64 * t27580 * t8875 + 0.40372756094140390856e-3_f64 * t125474 - 0.40372756094140390856e-3_f64 * t27714 * t8875 + t118002 / 2304.0_f64 - t118005 - 0.40372756094140390856e-3_f64 * t118007 - 0.40372756094140390856e-3_f64 * t27704 * t32429;
    t125482
}
