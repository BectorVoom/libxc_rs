//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1392/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1392(t101826: f64, t101828: f64, t101830: f64, t101832: f64, t101833: f64, t101835: f64, t101837: f64, t101839: f64, t101840: f64, t101841: f64, t102804: f64, t102813: f64, t102816: f64, t102820: f64, t102828: f64, t102833: f64, t102836: f64, t102839: f64, t102840: f64, t103794: f64, t103930: f64, t187: f64) -> f64 {
    let t103934 = t101826 - t101828 - t101830 - t101832 - t101833 + t101835 - t101837 - t101839 - t101840 - t101841 + t187 * (t102804 + t102840 + t103794 + t103930) + t102813 + t102816 + t102820 + t102828 + t102833 - t102836 - t102839;
    t103934
}
