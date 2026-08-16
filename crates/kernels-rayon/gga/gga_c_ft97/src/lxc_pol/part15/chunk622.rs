//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 622/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk622(t255: f64, t9952: f64, t258: f64, t9570: f64, t9577: f64, t1162: f64, t2399: f64, t89: f64, t676: f64, t1160: f64, t2492: f64, t265: f64, t9895: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14080 = t9952 * t255;
    let t14081 = t258 * t9570;
    let t14098 = t258 * t9577;
    let t14114 = t89 * t2399 * t1162;
    let t14127 = t676 * t255;
    let t14159 = t2492 * t1160;
    let t14163 = t9895 * t265;
    (t14080, t14081, t14098, t14114, t14127, t14159, t14163)
}
