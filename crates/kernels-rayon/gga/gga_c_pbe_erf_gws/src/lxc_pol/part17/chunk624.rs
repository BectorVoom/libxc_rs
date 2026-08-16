//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 624/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk624(t2014: f64, t2015: f64, t2745: f64, t2746: f64, t2748: f64, t2751: f64, t2755: f64, t2758: f64, t2788: f64, t2792: f64, t2794: f64, t2795: f64, t2798: f64, t2802: f64, t2806: f64, t2808: f64, t2818: f64, t2828: f64) -> f64 {
    let t2977 = -t2745 + t2746 + t2748 - t2751 + t2755 - t2758 - t2788 + t2792 - t2794 + t2795 + t2014 + 4.0_f64 / 3.0_f64 * t2015 + t2798 + t2802 - t2806 + t2808 + t2818 + t2828;
    t2977
}
