//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1123/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1123(t2228: f64, t2242: f64, t6751: f64, t6832: f64, t375: f64, t6125: f64, t2417: f64, t6336: f64, t6707: f64, t4379: f64, t6: f64, t6322: f64, t6563: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20160 = t2242 * t2228;
    let t20162 = t6832 * t6751;
    let t20173 = 1.0_f64 / t6125 / t375;
    let t20174 = t2417 * t2417;
    let t20181 = t6336 * t6707 / 24.0_f64;
    let t20182 = t6 * t4379;
    let t20188 = 3.0_f64 / 8.0_f64 * t6322 * t6563;
    (t20160, t20162, t20173, t20174, t20181, t20182, t20188)
}
