//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 696/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk696(t10122: f64, t874: f64, t1445: f64, t574: f64, t2877: f64, t3149: f64, t3153: f64, t10497: f64, t895: f64, t10340: f64, t1562: f64, t2854: f64, t3116: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12904 = t10122 * t874;
    let t12905 = t1445 * t12904;
    let t12906 = t574 * t12905;
    let t12909 = 0.35750489951850426669e0_f64 * t3149 * t2877;
    let t12911 = 0.35750489951850426669e0_f64 * t3153 * t2877;
    let t12912 = t895 * t10497;
    let t12914 = t10340 * t874;
    let t12915 = t1445 * t12914;
    let t12916 = t1562 * t12915;
    let t12918 = t2854 * t3116;
    (t12904, t12905, t12906, t12909, t12911, t12912, t12914, t12915, t12916, t12918)
}
