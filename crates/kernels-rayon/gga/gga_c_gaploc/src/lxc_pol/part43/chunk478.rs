//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 478/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk478(t6508: f64, t7892: f64, t447: f64, t986: f64, t2366: f64, t2754: f64, t555: f64, t1570: f64) -> (f64, f64, f64, f64, f64) {
    let t7893 = t6508 * t7892;
    let t7905 = t986 * t447;
    let t7906 = t2366 * t7905;
    let t7930 = t555 * t2754;
    let t7937 = t1570 * t986;
    (t7893, t7905, t7906, t7930, t7937)
}
