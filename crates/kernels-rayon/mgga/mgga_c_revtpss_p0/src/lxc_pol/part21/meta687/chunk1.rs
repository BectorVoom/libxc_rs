//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2506/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2506(t1208: f64, t12689: f64, t225: f64, t480: f64, t3671: f64, t3672: f64, t371: f64, t676: f64, t12625: f64, t458: f64, t456: f64, t43813: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44831 = t12689 * t1208;
    let t44832 = t44831 * t225;
    let t44833 = t44832 * t480;
    let t44838 = t3671 * t371 * t676 * t3672;
    let t44841 = 1.0_f64 / t12625 / t458;
    let t44842 = t456 * t44841;
    let t44843 = t44842 * t225;
    let t44865 = 0.15365432098765432099e0_f64 * t43813;
    (t44831, t44832, t44833, t44838, t44842, t44843, t44865)
}
