//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 822/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk822(t2380: f64, t274: f64, t801: f64, t10580: f64, t2: f64, t2347: f64, t852: f64, t2360: f64, t2842: f64, t668: f64, t2770: f64, t319: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14844 = t2380 * t801 * t274;
    let t14961 = t10580 * t2;
    let t15042 = t852 * t2347;
    let t15047 = t852 * t2360;
    let t15182 = t2842 * t668;
    let t15229 = t2770 * t319;
    (t14844, t14961, t15042, t15047, t15182, t15229)
}
