//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 896/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk896(t1621: f64, t7785: f64, t1620: f64, t2637: f64, t7136: f64, t5312: f64, t2825: f64, t586: f64, t593: f64, t1037: f64, t5470: f64, t1627: f64, t2593: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7786 = t1621 * t7785;
    let t7788 = 4.0_f64 / 15.0_f64 * t1620 * t7786;
    let t7790 = 8.0_f64 / 15.0_f64 * t7136 * t2637;
    let t7792 = 8.0_f64 / 15.0_f64 * t5312 * t2637;
    let t7793 = t2825 * t586;
    let t7795 = 8.0_f64 / 45.0_f64 * t7793 * t593;
    let t7797 = 4.0_f64 / 45.0_f64 * t5470 * t1037;
    let t7799 = 16.0_f64 / 45.0_f64 * t1627 * t2593;
    (t7788, t7790, t7792, t7795, t7797, t7799)
}
