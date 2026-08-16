//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 618/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk618(t8557: f64, t8558: f64, t1853: f64, t432: f64, t1852: f64, t452: f64, t1859: f64, t1882: f64, t1643: f64, t1866: f64, t499: f64, t110: f64, t447: f64, t7973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8559 = t8557 * t8558;
    let t8562 = t1853 * t432;
    let t8564 = t452 * t1852 * t8562;
    let t8567 = t1882 * t1859;
    let t8570 = t1866 * t499 * t1643;
    let t8574 = t447 * t110 * t7973;
    (t8559, t8562, t8564, t8567, t8570, t8574)
}
