//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 937/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk937(t13544: f64, t13550: f64, t13559: f64, t13569: f64, t14532: f64, t14554: f64, t2417: f64, t4068: f64, t688: f64, t9558: f64, t9560: f64, t9562: f64, t9564: f64) -> f64 {
    let t14555 = -0.9628722222222222222e-1_f64 * t9562 - 0.10591594444444444444e1_f64 * t13544 + 0.28886166666666666666e0_f64 * t13569 + 0.57772333333333333332e0_f64 * t13550 - 0.86658499999999999998e0_f64 * t13559 - 0.234754e0_f64 * t14532 * t688 - 0.117377e0_f64 * t4068 * t2417 - 0.12838296296296296296e0_f64 * t9558 + 0.4814361111111111111e-1_f64 * t9564 + 0.3209574074074074074e-1_f64 * t9560 + t14554;
    t14555
}
