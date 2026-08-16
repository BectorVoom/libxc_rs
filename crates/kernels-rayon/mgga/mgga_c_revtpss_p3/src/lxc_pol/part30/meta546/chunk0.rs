//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1985/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1985(t675: f64, t886: f64, t11006: f64, t256: f64, t10115: f64, t251: f64, t2410: f64, t11238: f64, t196: f64, t3800: f64, t12625: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41040 = t675 * t886;
    let t41077 = 1.0_f64 / t11006 / t256;
    let t41117 = t10115 * t251;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0_f64 / t41153;
    let t42859 = 1.0_f64 / t11238 / t196;
    let t44125 = t3800 * t3800;
    let t44126 = 1.0_f64 / t44125;
    let t44841 = 1.0_f64 / t12625 / t458;
    (t41040, t41077, t41117, t41154, t42859, t44126, t44841)
}
