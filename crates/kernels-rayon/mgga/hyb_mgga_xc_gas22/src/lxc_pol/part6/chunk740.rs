//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 740/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk740(t43: f64, t1895: f64, t1898: f64, t3814: f64, t1903: f64, t575: f64, t3804: f64, t578: f64, t1888: f64, t3006: f64, t572: f64) -> (f64, f64, f64, f64, f64) {
    let t45 = 0.135e1_f64 < t43;
    let t3832 = t1895 * t1898 * t3814;
    let t3836 = t575 * t1903 * t3814;
    let t3840 = t575 * t578 * t3804;
    let t3843 = t1888 + t3006 / 81.0_f64 - t572 * t3832 / 81.0_f64 + t572 * t3836 / 27.0_f64 - t572 * t3840 / 54.0_f64;
    let t3844 = piecewise3(t45, t3843, 0.0_f64);
    (t3832, t3836, t3840, t3843, t3844)
}
