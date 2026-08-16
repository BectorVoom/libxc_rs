//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1415/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1415(t2487: f64, t34321: f64, t6711: f64, t31190: f64, t31213: f64, t31215: f64, t31217: f64, t34950: f64, t34953: f64, t34954: f64, t34957: f64, t34959: f64, t34962: f64, t34964: f64, t34967: f64, t34970: f64, t34973: f64, t34976: f64) -> f64 {
    let t34979 = 0.87421871174939309262e2_f64 * t2487 * t6711 * t34321;
    let t34980 = t34950 + t34953 - t31190 - t34954 - t31213 - t31215 + t31217 + t34957 - t34959 + t34962 - t34964 - t34967 - t34970 + t34973 - t34976 + t34979;
    t34980
}
