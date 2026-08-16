//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1134/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1134(t1882: f64, t35990: f64, t143100: f64, t35825: f64, t2: f64, t35972: f64, t2665: f64, t6317: f64, t684: f64, t153372: f64, t27: f64, t799: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t153388 = t1882 * t35990;
    let t153390 = t143100 * t35825;
    let t153392 = t2 * t35972;
    let t153395 = t6317 * t2665 * t153392 * t684;
    let t153399 = t89 * t27 * t799 * t153372;
    (t153388, t153390, t153395, t153399)
}
