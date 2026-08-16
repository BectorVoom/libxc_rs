//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 565/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk565(t1852: f64, t4551: f64, t83: f64, t447: f64, t925: f64, t986: f64, t110: f64, t4462: f64, t1866: f64, t4454: f64, t1871: f64, t4436: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4552 = t1852 * t4551;
    let t4553 = t83 * t4552;
    let t4557 = t447 * t986 * t925;
    let t4561 = t447 * t110 * t4462;
    let t4565 = t1866 * t110 * t4454;
    let t4569 = t1871 * t110 * t4436;
    (t4552, t4553, t4557, t4561, t4565, t4569)
}
