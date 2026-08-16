//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1213/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1213(t1912: f64, t19652: f64, t3717: f64, t11509: f64, t5633: f64, t3144: f64, t34409: f64, t11329: f64, t8885: f64, t11379: f64, t11381: f64, t25953: f64) -> (f64, f64, f64, f64, f64) {
    let t34971 = t19652 * t3717 * t1912;
    let t34973 = t11509 * t5633;
    let t34975 = t34409 * t3144;
    let t34977 = t11329 * t8885;
    let t34980 = t25953 * t11379 * t11381;
    (t34971, t34973, t34975, t34977, t34980)
}
