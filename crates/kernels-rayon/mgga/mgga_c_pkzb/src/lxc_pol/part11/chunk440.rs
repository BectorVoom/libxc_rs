//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 440/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk440(t2003: f64, t66: f64, t132: f64, t747: f64, t288: f64, t749: f64) -> (f64, f64, f64, f64) {
    let t2004 = t66 * t2003;
    let t2016 = 1.0_f64 / t747 / t132;
    let t2018 = 1.0_f64 / t749 / t288;
    let t2019 = t2016 * t2018;
    (t2004, t2016, t2018, t2019)
}
