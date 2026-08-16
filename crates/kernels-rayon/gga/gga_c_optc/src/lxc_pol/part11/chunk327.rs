//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 327/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk327(t1327: f64, t1340: f64, t966: f64, t968: f64, t975: f64, t977: f64) -> (f64, f64) {
    let t1422 = -t966 - 0.19388333333333333333e1_f64 * t1327 - t968 - 0.12315e-2_f64 * t1340;
    let t1426 = -t975 - 0.72691666666666666667e3_f64 * t1327 - t977 - 0.78666666666666666667e2_f64 * t1340;
    (t1422, t1426)
}
