//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 693/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk693(t1866: f64, t4454: f64, t986: f64, t4417: f64, t979: f64, t8210: f64, t3193: f64, t942: f64, t3194: f64, t8518: f64, t4431: f64, t1903: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20200 = t1866 * t986 * t4454;
    let t20203 = t4417 * t979;
    let t20204 = t8210 * t20203;
    let t20205 = t3193 * t20204;
    let t20208 = t4417 * t942;
    let t20209 = t3194 * t20208;
    let t20210 = t8518 * t20209;
    let t20213 = t4431 * t942;
    let t20214 = t1903 * t20213;
    (t20200, t20203, t20204, t20205, t20208, t20209, t20210, t20213, t20214)
}
