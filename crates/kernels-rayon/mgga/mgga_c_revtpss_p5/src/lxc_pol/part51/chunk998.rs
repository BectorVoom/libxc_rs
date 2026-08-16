//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 998/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk998(t31959: f64, t33803: f64, t1089: f64, t1668: f64, t31935: f64, t1976: f64, t7810: f64, t31892: f64, t1646: f64, t373: f64, t372: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33804 = t31959 * t33803;
    let t33808 = t31935 * t1668 * t1089;
    let t33811 = t1976 * t7810;
    let t33812 = t31892 * t33811;
    let t33815 = t373 * t1646;
    let t33816 = t372 * t33815;
    let t33817 = t371 * t33816;
    (t33804, t33808, t33811, t33812, t33815, t33817)
}
