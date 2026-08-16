//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 488/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk488(t2843: f64, t2844: f64, t296: f64, t684: f64, t835: f64, t882: f64, t2413: f64, t319: f64, t2404: f64, t295: f64) -> (f64, f64, f64, f64, f64) {
    let t2845 = t2843 * t2844;
    let t2846 = t296 * t2845;
    let t2850 = t835 * t882 * t684;
    let t2854 = t835 * t319 * t2413;
    let t2857 = t2404 * t295;
    (t2845, t2846, t2850, t2854, t2857)
}
