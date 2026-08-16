//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1003/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1003(t13803: f64, t13804: f64, t13809: f64, t13818: f64, t219: f64, t1634: f64, t73: f64, t3346: f64, t5371: f64, t1206: f64, t4397: f64, t4452: f64) -> (f64, f64, f64, f64) {
    let t13821 = (t13803 + t13804 + t13809 + t13818) * t219;
    let t13827 = t1634 * t73;
    let t13834 = t3346 * t5371;
    let t13835 = t13834 * t1206;
    let t13838 = t4452 * t4397;
    (t13821, t13827, t13835, t13838)
}
