//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3239/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3239(t12584: f64, t12587: f64, t1832: f64, t3798: f64, t44126: f64, t5023: f64, t5501: f64, t57846: f64, t57849: f64, t57851: f64, t57853: f64, t57856: f64, t57860: f64, t57863: f64, t57907: f64, t57911: f64) -> f64 {
    let t60139 = -6.0_f64 * t12584 * t1832 * t44126 * t5023 + 6.0_f64 * t12587 * t3798 * t5023 * t5501 + t57846 + t57849 + t57851 + t57853 + t57856 + t57860 - t57863 - t57907 + t57911;
    t60139
}
