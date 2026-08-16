//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1306/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1306(t11423: f64, t51351: f64, t3116: f64, t54373: f64, t3065: f64, t3840: f64, t6645: f64, t3879: f64, t2134: f64, t3759: f64, t51214: f64, t11516: f64, t14011: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56912 = t51351 * t11423;
    let t56914 = t3116 * t54373;
    let t56916 = t3065 * t3840;
    let t56917 = t6645 * t56916;
    let t56919 = t3065 * t3879;
    let t56920 = t2134 * t56919;
    let t56922 = t51214 * t3759;
    let t56924 = t14011 * t11516;
    (t56912, t56914, t56917, t56920, t56922, t56924)
}
