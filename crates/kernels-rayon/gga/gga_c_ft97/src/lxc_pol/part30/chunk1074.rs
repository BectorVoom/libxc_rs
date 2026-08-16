//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1074/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1074(t1882: f64, t35636: f64, t35596: f64, t35601: f64, t110401: f64, t14127: f64, t141817: f64, t141820: f64, t141834: f64, t141850: f64, t141852: f64, t150133: f64, t150233: f64, t151051: f64, t151387: f64, t1901: f64, t242: f64, t24429: f64, t24668: f64, t2574: f64, t265: f64, t28023: f64, t28108: f64, t28145: f64, t35323: f64, t3821: f64, t3972: f64, t446: f64, t6088: f64, t6194: f64, t6852: f64, t6861: f64, t729: f64, t7484: f64, t7560: f64, t762: f64, t773: f64) -> f64 {
    let t151712 = t1882 * t35636;
    let t151715 = t1882 * t35596;
    let t151725 = t1882 * t35601;
    let t151760 = -4.0_f64 / 3.0_f64 * t1901 * t14127 * t24668 * t28108 - 4.0_f64 / 3.0_f64 * t1901 * t110401 * t28145 + t141817 / 9.0_f64 - t141820 - t151712 / 9.0_f64 - t141834 / 27.0_f64 - t151715 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t242 * t151387 + t446 * t729 * t762 * t7484 * t3972 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t151725 - t446 * t242 * t151051 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t729 * t28023 * t6088 - t446 * t729 * t7560 * t3821 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t2574 * t773 * t35323 + 4.0_f64 / 3.0_f64 * t446 * t2574 * t265 * t150133 + 4.0_f64 / 3.0_f64 * t446 * t2574 * t6194 * t6852 + 4.0_f64 / 3.0_f64 * t446 * t2574 * t265 * t150233 - 2.0_f64 / 9.0_f64 * t141850 + t141852 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t729 * t24429 * t6861;
    t151760
}
