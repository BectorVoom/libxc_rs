//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1383/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1383<F: Float>(t291: F, t77454: F, t77467: F, t10811: F, t10828: F, t14263: F, t14271: F, t14276: F, t14337: F, t1580: F, t1581: F, t21238: F, t21259: F, t21309: F, t21312: F, t21321: F, t2905: F, t2930: F, t2932: F, t311: F, t5742: F, t5775: F, t5790: F, t5794: F, t59941: F, t69276: F, t77133: F, t77135: F, t77138: F, t77139: F, t77226: F, t77229: F, t77232: F, t77427: F, t77440: F) -> (F, F) {
    let t77470 = F::cast_from(0.621814e-1_f64) * (t77454 + t77467) * t291;
    let t77471 = t77133 - t77135 - t77138 - F::cast_from(0.62337092780453269531e3_f64) * t10828 * t5794 * t5790 + F::cast_from(0.2077903092681775651e3_f64) * t14337 * t21312 + F::cast_from(0.69263436422725855036e2_f64) * t2930 * t69276 * t1580 - F::cast_from(24.0_f64) * t14276 * t21321 + F::cast_from(0.51947577317044391277e2_f64) * t2930 * t77139 * t2932 + F::cast_from(24.0_f64) * t14271 * t21259 - t77226 + t77229 + t77232 + F::cast_from(0.12414243100625616072e5_f64) * t10811 * t59941 * t5742 - F::cast_from(0.14035736694323150897e2_f64) * t14263 * t21309 + F::cast_from(0.21053605041484726346e2_f64) * t2930 * t5775 * t5790 - F::cast_from(0.46785788981077169656e1_f64) * t2905 * t1581 * t21238 - F::cast_from(0.310907e-1_f64) * (t77427 + t77440) * t311 + t77470;
    (t77470, t77471)
}
