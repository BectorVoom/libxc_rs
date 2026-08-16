//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1026/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1026(t32809: f64, t32810: f64, t43494: f64, t1: f64, t10083: f64, t1022: f64, t2084: f64, t787: f64, t42944: f64, t701: f64) -> (f64, f64, f64) {
    let t43592 = 0.85801175884441024004e1_f64 * t32809 * t32810 * t43494;
    let t43597 = 0.21450293971110256001e2_f64 * t787 * t2084 * t1022 * t1 * t10083;
    let t43598 = t42944 * t701;
    (t43592, t43597, t43598)
}
