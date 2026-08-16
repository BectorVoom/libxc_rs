//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1002/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1002(t1524: f64, t355: f64, t1083: f64, t1980: f64, t7458: f64, t1089: f64, t15995: f64, t2090: f64, t598: f64, t535: f64, t7457: f64, t7459: f64) -> (f64, f64, f64, f64, f64) {
    let t33883 = t355 * t1524;
    let t33884 = t1083 * t33883;
    let t33886 = t1980 * t7458 * t33884;
    let t33890 = t598 * t1089 * t15995 * t2090;
    let t33894 = t7457 * t7458 * t535 * t7459;
    (t33883, t33884, t33886, t33890, t33894)
}
