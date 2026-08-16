//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1288/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1288(t10450: f64, t1289: f64, t1318: f64, t2004: f64, t214: f64, t2154: f64, t23802: f64, t23814: f64, t23817: f64, t23828: f64, t23831: f64, t23834: f64, t23853: f64, t23856: f64, t23883: f64, t23886: f64, t23891: f64, t23896: f64, t23938: f64, t23943: f64, t2986: f64, t3947: f64, t3990: f64, t684: f64, t686: f64, t766: f64, t8456: f64) -> f64 {
    let t27935 = t23802 / 72.0_f64 - 5.0_f64 / 432.0_f64 * t23814 - t23817 / 48.0_f64 + t23828 / 72.0_f64 - t23831 / 96.0_f64 - t23834 / 72.0_f64 - t23853 / 96.0_f64 + t23856 / 24.0_f64 + t684 * t2986 * t8456 * t1318 / 16.0_f64 + t684 * t2986 * t686 * t1289 * t214 / 16.0_f64 - 3.0_f64 / 32.0_f64 * t2004 * t3947 - 3.0_f64 / 32.0_f64 * t10450 * t766 - 3.0_f64 / 64.0_f64 * t3990 * t2154 - t23883 / 36.0_f64 - t23886 / 72.0_f64 - 7.0_f64 / 216.0_f64 * t23891 + t23896 / 18.0_f64 + t23938 / 24.0_f64 + t23943 / 54.0_f64;
    t27935
}
