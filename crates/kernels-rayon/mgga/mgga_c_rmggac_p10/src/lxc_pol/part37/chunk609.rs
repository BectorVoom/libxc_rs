//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 609/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk609(t15445: f64, t209: f64, t605: f64, t698: f64, t515: f64, t1971: f64, t1970: f64, t15187: f64, t15189: f64, t15191: f64, t2144: f64, t9540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15446 = 0.39914139006212695214e-1_f64 * t15445;
    let t15448 = t698 * t605 * t209;
    let t15449 = t515 * t15448;
    let t15450 = t1971 * t15449;
    let t15451 = t1970 * t15450;
    let t15452 = 0.42564599893297839398e-5_f64 * t15451;
    let t15453 = 0.20455996240684006298e-1_f64 * t15187;
    let t15454 = 0.2727466165424534173e-1_f64 * t15189;
    let t15455 = 0.13637330827122670865e-1_f64 * t15191;
    let t15456 = t2144 * t9540;
    (t15446, t15450, t15452, t15453, t15454, t15455, t15456)
}
