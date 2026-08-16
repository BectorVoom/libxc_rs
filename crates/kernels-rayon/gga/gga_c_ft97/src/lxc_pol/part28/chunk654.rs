//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 654/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk654(t1307: f64, t3255: f64, t452: f64, t488: f64, t3238: f64, t5644: f64, t1825: f64, t6478: f64, t5617: f64, t979: f64, t1882: f64, t6488: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26410 = t1307 * t3255;
    let t26412 = t452 * t488 * t26410;
    let t26416 = t452 * t3238 * t5644;
    let t26420 = t452 * t1825 * t6478;
    let t26423 = t5617 * t979;
    let t26425 = t452 * t488 * t26423;
    let t26428 = t1882 * t6488;
    (t26410, t26412, t26416, t26420, t26423, t26425, t26428)
}
