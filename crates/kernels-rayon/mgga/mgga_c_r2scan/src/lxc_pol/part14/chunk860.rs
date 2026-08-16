//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 860/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk860(t1732: f64, t2758: f64, t5754: f64, t5761: f64, t5766: f64, t5770: f64, t5774: f64, t5901: f64, t5907: f64, t5908: f64, t5910: f64, t5912: f64, t5919: f64) -> f64 {
    let t7817 = t2758 * t1732;
    let t7822 = -0.10005107252466666666e-2_f64 * t7817 + t5901 - t5754 + t5907 + 0.65061487801810439052e-1_f64 * t5908 + 0.1301229756036208781e0_f64 * t5910 + 0.38527786510141256862e1_f64 * t5912 + t5761 + t5766 + t5770 - t5774 + t5919;
    t7822
}
