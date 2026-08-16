//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 985/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk985(t1251: f64, t1552: f64, t3668: f64, t133: f64, t34038: f64, t34080: f64, t1576: f64, t3671: f64, t169: f64, t242: f64, t30129: f64, t10229: f64, t700: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34087 = t1552 * t3668 * t1251;
    let t34158 = t133 * t34038;
    let t34162 = t133 * t34080;
    let t34210 = t3671 * t1576;
    let t34237 = t169 * t30129 * t242;
    let t34244 = t169 * t10229 * t700;
    (t34087, t34158, t34162, t34210, t34237, t34244)
}
