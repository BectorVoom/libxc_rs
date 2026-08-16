//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 611/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk611<F: Float>(t15443: F, t3194: F, t5928: F, t209: F, t605: F, t698: F, t515: F, t1971: F, t1970: F, t15187: F, t15189: F, t15191: F) -> (F, F, F, F, F, F, F) {
    let t15444 = F::cast_from(0.59871208509319042821e-1_f64) * t15443;
    let t15445 = t5928 * t3194;
    let t15446 = F::cast_from(0.39914139006212695214e-1_f64) * t15445;
    let t15448 = t698 * t605 * t209;
    let t15449 = t515 * t15448;
    let t15450 = t1971 * t15449;
    let t15451 = t1970 * t15450;
    let t15452 = F::cast_from(0.42564599893297839398e-5_f64) * t15451;
    let t15453 = F::cast_from(0.20455996240684006298e-1_f64) * t15187;
    let t15454 = F::cast_from(0.2727466165424534173e-1_f64) * t15189;
    let t15455 = F::cast_from(0.13637330827122670865e-1_f64) * t15191;
    (t15444, t15446, t15450, t15452, t15453, t15454, t15455)
}
