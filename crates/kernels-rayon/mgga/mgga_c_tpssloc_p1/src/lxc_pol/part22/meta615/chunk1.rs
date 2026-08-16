//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2144/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2144(t52057: f64, t15418: f64, t1714: f64, t11557: f64, t4889: f64, t1174: f64, t1716: f64, t2402: f64, t4930: f64, t698: f64, t44620: f64, t461: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52058 = 0.37037037037037037036e-3_f64 * t52057;
    let t52059 = t15418 * t1714;
    let t52074 = t4889 * t11557;
    let t52081 = t1174 * t2402 * t1716;
    let t52084 = t1174 * t698 * t4930;
    let t52085 = 0.55555555555555555554e-3_f64 * t52084;
    let t52096 = t60 * t44620 * t461;
    (t52058, t52059, t52074, t52081, t52085, t52096)
}
