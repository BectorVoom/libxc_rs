//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 860/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk860(t1390: f64, t6844: f64, t828: f64, t124: f64, t6836: f64, t800: f64, t1414: f64, t6816: f64, t1882: f64) -> (f64, f64, f64, f64, f64) {
    let t6846 = t1390 * t828 * t6844;
    let t6849 = t124 * t6836;
    let t6850 = t800 * t6849;
    let t6856 = t1414 * t828 * t6816;
    let t6861 = t1882 * t1882;
    (t6846, t6849, t6850, t6856, t6861)
}
