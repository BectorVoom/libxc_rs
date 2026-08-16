//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 993/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk993(t2242: f64, t3909: f64, t3780: f64, t4394: f64, t20839: f64, t3816: f64, t1114: f64, t3747: f64, t6643: f64, t3916: f64, t6644: f64, t11609: f64, t2118: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36340 = t2242 * t3909;
    let t36612 = t3780 * t4394;
    let t36626 = t20839 * t3816;
    let t36641 = t1114 * t3747 * t6643;
    let t36659 = t3916 * t6644;
    let t36666 = t2118 * t11609;
    (t36340, t36612, t36626, t36641, t36659, t36666)
}
