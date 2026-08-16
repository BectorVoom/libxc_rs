//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1137/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1137(t143040: f64, t143112: f64, t28741: f64, t33822: f64, t24980: f64, t2862: f64, t28776: f64, t33868: f64, t2665: f64, t28755: f64, t3746: f64, t152799: f64, t6317: f64, t99559: f64) -> (f64, f64, f64, f64) {
    let t153435 = t143040 * t143112 * t33822 * t28741;
    let t153439 = t24980 * t2862 * t33868 * t28776;
    let t153443 = t28755 * t2665 * t33868 * t3746;
    let t153449 = t6317 * t99559 * t152799;
    (t153435, t153439, t153443, t153449)
}
