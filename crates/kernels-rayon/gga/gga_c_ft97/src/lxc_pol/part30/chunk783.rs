//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 783/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk783(t2665: f64, t33868: f64, t684: f64, t6317: f64, t7611: f64, t824: f64, t2781: f64, t1486: f64, t193: f64, t2035: f64, t7590: f64, t811: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33870 = t2665 * t33868 * t684;
    let t33871 = t6317 * t33870;
    let t33873 = t7611 * t824;
    let t33874 = t2781 * t33873;
    let t33876 = t1486 * t193 * t33874;
    let t33885 = t2035 * t7590 * t811;
    (t33870, t33871, t33873, t33874, t33876, t33885)
}
