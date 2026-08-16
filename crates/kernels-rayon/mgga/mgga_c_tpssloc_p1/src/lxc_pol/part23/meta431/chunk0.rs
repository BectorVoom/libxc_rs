//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1267/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1267(t15438: f64, t19095: f64, t19083: f64, t4993: f64, t18392: f64, t5024: f64, t1226: f64, t22115: f64, t1227: f64, t21776: f64, t248: f64, t3521: f64) -> (f64, f64, f64, f64, f64) {
    let t72248 = t15438 * t19095;
    let t72251 = t19083 * t4993;
    let t72253 = t5024 * t18392;
    let t72255 = t22115 * t1226;
    let t72273 = t1227 * t248 * t3521 * t21776;
    (t72248, t72251, t72253, t72255, t72273)
}
