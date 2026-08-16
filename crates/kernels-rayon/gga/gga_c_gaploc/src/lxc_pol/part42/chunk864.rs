//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 864/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk864(t33331: f64, t33332: f64, t45369: f64, t1457: f64, t44995: f64, t6060: f64, t13691: f64, t15766: f64, t13077: f64, t8634: f64, t11765: f64, t2718: f64) -> (f64, f64, f64, f64, f64) {
    let t45372 = 0.13803453343411469884e3_f64 * t33331 * t33332 * t45369;
    let t45375 = 0.21450293971110256001e1_f64 * t6060 * t1457 * t44995;
    let t45377 = 0.21450293971110256001e1_f64 * t15766 * t13691;
    let t45379 = 0.71500979903700853338e0_f64 * t13077 * t8634;
    let t45381 = 0.35750489951850426669e0_f64 * t2718 * t11765;
    (t45372, t45375, t45377, t45379, t45381)
}
