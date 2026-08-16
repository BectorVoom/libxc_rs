//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1310/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1310(t10538: f64, t30523: f64, t12433: f64, t1611: f64, t1112: f64, t30867: f64, t13281: f64, t1617: f64, t3859: f64, t34344: f64, t34346: f64, t34351: f64, t34353: f64, t34356: f64, t34359: f64, t34361: f64, t34364: f64, t34367: f64, t34370: f64, t34373: f64) -> (f64, f64, f64, f64, f64) {
    let t38082 = 12.0_f64 * t30523 * t10538;
    let t38086 = 2.0_f64 * t1611 * t12433;
    let t38088 = 2.0_f64 * t30867 * t1112;
    let t38093 = 24.0_f64 * t13281 * t3859 * t1617;
    let t38118 = 0.2318836277704281739e-4_f64 * t34344 + 0.86880925264517213544e-4_f64 * t34346 - 0.10136107947527008247e-2_f64 * t34351 - 0.80966145833333333339e-4_f64 * t34353 + 0.11584123368602295139e-4_f64 * t34356 + 0.11584123368602295139e-4_f64 * t34359 - 0.11382560960801989336e-6_f64 * t34361 + 0.2023819338830593704e-6_f64 * t34364 + 0.4637672555408563478e-4_f64 * t34367 - 0.9275345110817126956e-4_f64 * t34370 + 0.13672076938352463841e-4_f64 * t34373;
    (t38082, t38086, t38088, t38093, t38118)
}
