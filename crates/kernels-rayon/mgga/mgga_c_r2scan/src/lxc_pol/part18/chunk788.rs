//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 788/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk788(t2148: f64, t7629: f64, t7628: f64, t1398: f64, t5: f64, t966: f64, t2804: f64, t378: f64, t1707: f64, t898: f64, t1726: f64, t1727: f64, t956: f64) -> (f64, f64, f64, f64, f64) {
    let t7630 = t2148 * t7629;
    let t7632 = 0.23287303101564395622e-1_f64 * t7628 * t7630;
    let t7637 = t5 * t1398 * t966;
    let t7641 = 10.0_f64 / 3.0_f64 * t5 * t378 * t2804;
    let t7647 = t898 * t1707;
    let t7650 = t1726 * t956 * t1727;
    (t7632, t7637, t7641, t7647, t7650)
}
