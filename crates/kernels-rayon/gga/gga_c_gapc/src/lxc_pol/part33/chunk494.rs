//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 494/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk494(t2891: f64, t473: f64, t126: f64, t507: f64, t120: f64, t1007: f64, t518: f64, t2880: f64, t568: f64, t1539: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2892 = t473 * t2891;
    let t2894 = t126 * t507;
    let t2895 = t120 * t2894;
    let t2897 = t518 * t1007;
    let t2899 = t2880 * t568;
    let t2900 = t120 * t2899;
    let t2902 = t5 * t1539;
    (t2892, t2894, t2895, t2897, t2899, t2900, t2902)
}
