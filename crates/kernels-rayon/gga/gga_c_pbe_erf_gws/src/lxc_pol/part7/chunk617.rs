//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 617/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk617(t4791: f64, t1422: f64, t75: f64, t472: f64, t1218: f64, t1399: f64, t1327: f64, t414: f64, t1319: f64, t455: f64, t4623: f64, t470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4792 = 0.51947267698127589899e2_f64 * t4791;
    let t4793 = t1422 * t75;
    let t4794 = t4793 * t472;
    let t4795 = 0.17544670192365612213e1_f64 * t4794;
    let t4796 = t1399 * t1218;
    let t4797 = 0.35089340384731224426e1_f64 * t4796;
    let t4798 = t414 * t1327;
    let t4799 = 12.0_f64 * t4798;
    let t4800 = t1319 * t455;
    let t4801 = t4800 * t4623;
    let t4802 = t470 * t4801;
    (t4792, t4793, t4794, t4795, t4796, t4797, t4798, t4799, t4800, t4801, t4802)
}
