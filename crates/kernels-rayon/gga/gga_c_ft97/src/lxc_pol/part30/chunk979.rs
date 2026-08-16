//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 979/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk979(t1882: f64, t34120: f64, t34086: f64, t8392: f64, t34142: f64, t34136: f64, t7626: f64, t8232: f64, t7611: f64, t870: f64, t34130: f64, t2770: f64, t7662: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t143900 = t1882 * t34120;
    let t143902 = t8392 * t34086;
    let t143904 = t1882 * t34142;
    let t143924 = t1882 * t34136;
    let t143953 = 8.0_f64 / 27.0_f64 * t8232 * t7626;
    let t143989 = t870 * t7611;
    let t143998 = t1882 * t34130;
    let t144005 = t2770 * t7662;
    (t143900, t143902, t143904, t143924, t143953, t143989, t143998, t144005)
}
