//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 739/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk739(t309: f64, t310: f64, t311: f64, t305: f64, t296: f64, t413: f64, t4652: f64, t4664: f64, t4747: f64, t4751: f64, t4754: f64, t4756: f64, t4780: f64, t4784: f64, t4786: f64, t4790: f64, t4792: f64, t4795: f64, t4797: f64) -> (f64, f64, f64, f64, f64) {
    let t6072 = 1.0_f64 / t311 / t310 / t309;
    let t6073 = t305 * t6072;
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = 0.47400060215270560269e0_f64 * t6075;
    let t6077 = t4747 + t4751 + t4652 + t4754 + t4756 + t4664 - t6076 + t4780 - t4784 - t4786 - t4790 - t4792 - t4795 + t4797;
    (t6072, t6073, t6074, t6075, t6077)
}
