//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 961/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk961(t751: f64, t8269: f64, t5942: f64, t8424: f64, t2970: f64, t5931: f64, t5927: f64, t3013: f64, t671: f64, t1049: f64, t1985: f64, t5904: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26153 = t751 * t8269;
    let t26196 = t8424 * t5942;
    let t26204 = t2970 * t5931;
    let t26242 = t2970 * t5927;
    let t26308 = t3013 * t671;
    let t26314 = t1049 * t1985;
    let t26328 = t1049 * t5904;
    (t26153, t26196, t26204, t26242, t26308, t26314, t26328)
}
