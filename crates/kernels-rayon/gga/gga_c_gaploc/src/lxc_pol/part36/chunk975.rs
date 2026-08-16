//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 975/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk975(t43007: f64, t701: f64, t6066: f64, t7630: f64, t32809: f64, t32810: f64, t43494: f64, t1: f64, t10083: f64, t1022: f64, t2084: f64, t787: f64) -> (f64, f64, f64, f64) {
    let t43586 = t43007 * t701;
    let t43588 = t7630 * t6066 * t43586;
    let t43592 = 0.85801175884441024004e1_f64 * t32809 * t32810 * t43494;
    let t43597 = 0.21450293971110256001e2_f64 * t787 * t2084 * t1022 * t1 * t10083;
    (t43586, t43588, t43592, t43597)
}
