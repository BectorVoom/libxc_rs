//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 882/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk882(t1389: f64, t6616: f64, t28: f64, t6578: f64, t7150: f64, t1360: f64, t925: f64, t356: f64, t461: f64, t6681: f64, t6615: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34979 = t6616 * t1389;
    let t34980 = t28 * t34979;
    let t34985 = t6578 * t7150;
    let t34988 = t1360 * t925;
    let t34989 = t356 * t34988;
    let t34994 = t461 * t6681;
    let t35000 = t72 * t6615;
    (t34979, t34980, t34985, t34988, t34989, t34994, t35000)
}
