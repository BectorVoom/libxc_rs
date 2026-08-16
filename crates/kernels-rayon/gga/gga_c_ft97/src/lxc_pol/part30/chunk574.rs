//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 574/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk574(t2354: f64, t27483: f64, t446: f64, t1411: f64, t3758: f64, t1109: f64, t709: f64, t444: f64, t6032: f64, t3789: f64, t2446: f64, t3886: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27484 = t2354 * t27483;
    let t27485 = t446 * t27484;
    let t27487 = t3758 * t1411;
    let t27494 = sigma2 * t1109;
    let t27495 = t27494 * t709;
    let t27499 = t6032 * t444;
    let t27500 = t3789 * t27499;
    let t27501 = t2446 * t3886;
    (t27485, t27487, t27494, t27495, t27499, t27500, t27501)
}
