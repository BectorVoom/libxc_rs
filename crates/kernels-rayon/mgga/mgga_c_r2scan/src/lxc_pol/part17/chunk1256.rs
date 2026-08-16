//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1256/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1256(t322: f64, t44630: f64, t44641: f64, t12829: f64, t833: f64, t1013: f64, t1120: f64, t11220: f64, t12244: f64, t1300: f64, t2394: f64, t2941: f64, t2944: f64, t327: f64, t3506: f64, t3509: f64, t38839: f64, t41901: f64, t829: f64, t834: f64, t9676: f64) -> (f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t44642 = t44630 + t44641;
    let t44643 = piecewise3(t324, 0.0_f64, t44642);
    let t44646 = t12829 * t833;
    let t44661 = -0.128e1_f64 * t1300 * t3506 * t2941 - 0.128e1_f64 * t1300 * t1120 * t9676 - 0.128e1_f64 * t1300 * t12829 * t829 - 0.64e0_f64 * t44643 * t327 - 0.128e1_f64 * t44646 * t829 - 0.256e1_f64 * t41901 * t1013 - 0.256e1_f64 * t12244 * t2394 - 0.384e1_f64 * t38839 * t2944 - 0.128e1_f64 * t11220 * t2941 - 0.128e1_f64 * t3509 * t9676 - 0.64e0_f64 * t834 * t44643;
    (t44642, t44661)
}
