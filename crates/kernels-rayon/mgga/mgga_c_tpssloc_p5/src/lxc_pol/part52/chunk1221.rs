//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1221/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1221(t6517: f64, t7467: f64, t33094: f64, t8601: f64, t4028: f64, t8326: f64, t7676: f64, t5161: f64, t8489: f64, t1983: f64, t1799: f64, t3701: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33146 = t6517 * t7467;
    let t33148 = 2.0_f64 * t33094;
    let t33150 = 4.0_f64 * t8601 * t7467;
    let t33151 = t4028 * t8326;
    let t33152 = 2.0_f64 * t33151;
    let t33153 = t7676 * t8326;
    let t33154 = 2.0_f64 * t33153;
    let t33157 = t8489 * t5161;
    let t33158 = t1983 * t33157;
    let t33159 = t3701 * t1799;
    (t33146, t33148, t33150, t33152, t33154, t33157, t33158, t33159)
}
