//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 628/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk628(t118: f64, t14519: f64, t14521: f64, t15007: f64, t15146: f64, t15557: f64, t15559: f64, t15560: f64, t15561: f64, t15562: f64, t15870: f64, t15872: f64, t15885: f64) -> f64 {
    let t15887 = t15870 + 0.31062809106223861414e-2_f64 * t15146 - t15557 + t14519 + t15559 - t15560 - t15561 - t14521 + t15007 + t15562 - 0.39914139006212695214e-1_f64 * t118 * t15872 + t15885;
    t15887
}
