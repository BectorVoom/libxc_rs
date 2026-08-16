//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1621/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1621(t1246: f64, t19189: f64, t19120: f64, t493: f64, t1243: f64, t19045: f64, t3612: f64, t5011: f64, t1755: f64, t11881: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1758: f64, t18572: f64, t19166: f64, t19170: f64, t19174: f64, t19176: f64, t19180: f64, t3604: f64, t3610: f64, t470: f64, t494: f64, t4964: f64, t5064: f64, t5073: f64, t5076: f64, t5086: f64, t6168: f64, t6257: f64, t6265: f64) -> f64 {
    let t19190 = t19189 * t1246;
    let t19197 = t493 * t19120;
    let t19201 = t19045 * t1243;
    let t19203 = t3612 * t5011;
    let t19204 = t1755 * t19203;
    let t19207 = 6.0_f64 * t11881 * t19166 + t1201 * t6265 + 2.0_f64 * t1244 * t19170 + t1244 * t19174 + 2.0_f64 * t1244 * t19180 + t1244 * t19190 + t1247 * t19201 + t1249 * t6168 + 2.0_f64 * t1729 * t5086 + 2.0_f64 * t1758 * t4964 + t18572 * t494 + 2.0_f64 * t19176 * t3610 + t19197 * t470 + 4.0_f64 * t19204 * t3610 + 2.0_f64 * t3604 * t6257 + 2.0_f64 * t5064 * t5073 + 2.0_f64 * t5064 * t5076;
    t19207
}
