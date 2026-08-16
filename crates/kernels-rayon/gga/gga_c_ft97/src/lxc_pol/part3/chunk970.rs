//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 970/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk970(t1217: f64, t18986: f64, t4134: f64, t5206: f64, t2648: f64, t5304: f64, t1091: f64, t4162: f64, t10248: f64, t446: f64, t1212: f64, t505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18987 = t18986 * t1217;
    let t18989 = t5206 * t4134;
    let t18992 = t2648 * t5304;
    let t18997 = t1091 * t4162;
    let t18998 = t10248 * t18997;
    let t18999 = t446 * t18998;
    let t19001 = t1212 * t505;
    (t18987, t18989, t18992, t18997, t18999, t19001)
}
