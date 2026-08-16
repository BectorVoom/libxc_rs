//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 759/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk759(t2877: f64, t3153: f64, t10497: f64, t895: f64, t10340: f64, t874: f64, t1445: f64, t1562: f64, t2854: f64, t3116: f64, t1645: f64, t3133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12911 = 0.35750489951850426669e0_f64 * t3153 * t2877;
    let t12912 = t895 * t10497;
    let t12914 = t10340 * t874;
    let t12915 = t1445 * t12914;
    let t12916 = t1562 * t12915;
    let t12918 = t2854 * t3116;
    let t12919 = t1445 * t12918;
    let t12921 = 0.69017266717057349418e1_f64 * t1562 * t12919;
    let t12922 = t1645 * t3133;
    (t12911, t12912, t12914, t12915, t12916, t12918, t12919, t12921, t12922)
}
