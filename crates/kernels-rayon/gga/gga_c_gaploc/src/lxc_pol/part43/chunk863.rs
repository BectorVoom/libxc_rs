//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 863/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk863(t1415: f64, t42148: f64, t4446: f64, t10547: f64, t9333: f64, t12868: f64, t1580: f64, t12806: f64, t1562: f64, t4614: f64, t10533: f64, t20796: f64, t41738: f64) -> (f64, f64, f64, f64, f64) {
    let t42388 = 0.25025342966295298669e1_f64 * t1415 * t42148 * t4446;
    let t42390 = 0.50050685932590597338e1_f64 * t10547 * t9333;
    let t42392 = 0.11502877786176224903e2_f64 * t1580 * t12868;
    let t42395 = 0.82820720060468819301e2_f64 * t1562 * t4614 * t12806;
    let t42398 = 0.27606906686822939767e2_f64 * t20796 * t10533 * t41738;
    (t42388, t42390, t42392, t42395, t42398)
}
