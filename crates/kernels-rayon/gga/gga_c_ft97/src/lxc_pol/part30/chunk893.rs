//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 893/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk893(t1248: f64, t7584: f64, t2862: f64, t871: f64, t319: f64, t35833: f64, t6353: f64, t7045: f64, t840: f64, t296: f64, t36101: f64, t1091: f64, t7686: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36240 = t7584 * t1248;
    let t36242 = t2862 * t871 * t36240;
    let t36246 = t2862 * t319 * t35833;
    let t36250 = t840 * t6353 * t7045;
    let t36253 = t296 * t36101;
    let t36257 = t835 * t7686 * t1091;
    (t36240, t36242, t36246, t36250, t36253, t36257)
}
