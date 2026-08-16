//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 734/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk734(t1322: f64, t4607: f64, t2704: f64, t2718: f64, t4518: f64, t4521: f64, t4524: f64, t4529: f64, t4531: f64, t4533: f64, t456: f64, t1314: f64) -> (f64, f64, f64, f64) {
    let t4608 = t4607 * t1322;
    let t4619 = -0.34523333333333333333e1_f64 * t4518 + 0.23015555555555555556e1_f64 * t4521 - 0.26851481481481481482e1_f64 * t4524 - 0.93932222222222222223e0_f64 * t2704 + 0.73355e-1_f64 * t4529 - 0.14671e0_f64 * t4531 - 0.17116166666666666667e0_f64 * t4533 - 0.36793333333333333333e0_f64 * t2718;
    let t4620 = t4619 * t456;
    let t4623 = t1314 * t1322;
    (t4608, t4619, t4620, t4623)
}
