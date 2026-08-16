//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 794/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk794(t14125: f64, t68448: f64, t73722: f64, t3077: f64, t38973: f64, t68386: f64, t7248: f64, t8667: f64, t8830: f64, t9188: f64, t3352: f64, t8835: f64) -> (f64, f64, f64, f64, f64) {
    let t74319 = t68448 * t14125 * t73722;
    let t74321 = t38973 * t3077;
    let t74324 = t68386 * t7248 * t8667;
    let t74327 = t68386 * t9188 * t8830;
    let t74330 = t68386 * t3352 * t8835;
    (t74319, t74321, t74324, t74327, t74330)
}
