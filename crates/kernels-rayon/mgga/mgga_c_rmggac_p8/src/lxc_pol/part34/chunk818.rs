//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 818/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk818(t14025: f64, t9187: f64, t21708: f64, t9189: f64, t21709: f64, t9193: f64, t15238: f64, t9128: f64, t1550: f64, t2060: f64, t41059: f64, t14362: f64, t2411: f64, t3144: f64) -> (f64, f64, f64, f64, f64) {
    let t74703 = t14025 * t9187;
    let t74705 = t21708 * t74703 * t9189;
    let t74708 = t21708 * t21709 * t9193;
    let t74713 = 0.5987120850931904282e-1_f64 * t9128 * t15238;
    let t74716 = 0.5987120850931904282e-1_f64 * t1550 * t2060 * t41059;
    let t74718 = t2411 * t14362 * t3144;
    (t74705, t74708, t74713, t74716, t74718)
}
