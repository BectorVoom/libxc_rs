//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 831/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk831(t158: f64, t8839: f64, t8840: f64, t8847: f64, t8856: f64, t1029: f64, t133: f64, t1773: f64, t3401: f64, t568: f64, t2575: f64, t2632: f64) -> (f64, f64, f64, f64, f64) {
    let t8859 = (t8839 + t8840 + t8847 + t8856) * t158;
    let t8865 = t1029 * t133;
    let t8872 = t1773 * t3401;
    let t8873 = t8872 * t568;
    let t8876 = t2632 * t2575;
    (t8859, t8865, t8872, t8873, t8876)
}
