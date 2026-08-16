//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 998/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk998(t162: f64, t8087: f64, t1985: f64, t3565: f64, t10704: f64, t10706: f64, t10709: f64, t10712: f64, t10716: f64, t10719: f64, t10721: f64, t10724: f64, t10727: f64, t7979: f64, t7988: f64, t7992: f64, t8225: f64, t8231: f64, t8234: f64) -> (f64, f64) {
    let t10728 = t8087 * t162;
    let t10729 = t3565 * t1985;
    let t10731 = 24.0_f64 * t10728 * t10729;
    let t10732 = t8225 + t10704 - t8231 - t8234 + t7979 + t10706 + t10709 + t10712 + t10716 - t10719 + t10721 + t10724 + t10727 + t10731 + t7988 + t7992;
    (t10731, t10732)
}
