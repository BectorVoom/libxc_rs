//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 618/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk618(t4802: f64, t1215: f64, t1314: f64, t457: f64, t470: f64, t4664: f64, t4754: f64, t4756: f64, t4780: f64, t4784: f64, t4786: f64, t4790: f64, t4792: f64, t4795: f64, t4797: f64, t4799: f64) -> (f64, f64, f64, f64, f64) {
    let t4803 = 0.51947267698127589897e2_f64 * t4802;
    let t4805 = t1215 * t1314 * t457;
    let t4806 = t470 * t4805;
    let t4807 = 0.35089340384731224426e1_f64 * t4806;
    let t4808 = t4754 + t4756 + t4664 + t4780 - t4784 - t4786 - t4790 - t4792 - t4795 + t4797 - t4799 - t4803 + t4807;
    (t4803, t4805, t4806, t4807, t4808)
}
