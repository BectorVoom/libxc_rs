//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1383/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1383(t291: f64, t77454: f64, t77467: f64, t10811: f64, t10828: f64, t14263: f64, t14271: f64, t14276: f64, t14337: f64, t1580: f64, t1581: f64, t21238: f64, t21259: f64, t21309: f64, t21312: f64, t21321: f64, t2905: f64, t2930: f64, t2932: f64, t311: f64, t5742: f64, t5775: f64, t5790: f64, t5794: f64, t59941: f64, t69276: f64, t77133: f64, t77135: f64, t77138: f64, t77139: f64, t77226: f64, t77229: f64, t77232: f64, t77427: f64, t77440: f64) -> (f64, f64) {
    let t77470 = 0.621814e-1_f64 * (t77454 + t77467) * t291;
    let t77471 = t77133 - t77135 - t77138 - 0.62337092780453269531e3_f64 * t10828 * t5794 * t5790 + 0.2077903092681775651e3_f64 * t14337 * t21312 + 0.69263436422725855036e2_f64 * t2930 * t69276 * t1580 - 24.0_f64 * t14276 * t21321 + 0.51947577317044391277e2_f64 * t2930 * t77139 * t2932 + 24.0_f64 * t14271 * t21259 - t77226 + t77229 + t77232 + 0.12414243100625616072e5_f64 * t10811 * t59941 * t5742 - 0.14035736694323150897e2_f64 * t14263 * t21309 + 0.21053605041484726346e2_f64 * t2930 * t5775 * t5790 - 0.46785788981077169656e1_f64 * t2905 * t1581 * t21238 - 0.310907e-1_f64 * (t77427 + t77440) * t311 + t77470;
    (t77470, t77471)
}
