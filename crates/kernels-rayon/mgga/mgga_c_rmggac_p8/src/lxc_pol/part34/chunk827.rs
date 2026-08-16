//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 827/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk827(t14125: f64, t236: f64, t68884: f64, t8688: f64, t1509: f64, t68844: f64, t201: f64, t457: f64, t615: f64, t68876: f64, t13884: f64, t15296: f64) -> (f64, f64, f64, f64, f64) {
    let t74846 = t68884 * t14125 * t236 * t8688;
    let t74848 = t236 * t1509;
    let t74850 = t68844 * t14125 * t74848;
    let t74856 = t68876 * t14125 * t236 * t615 * t457 * t201;
    let t74858 = t15296 * t13884;
    (t74846, t74848, t74850, t74856, t74858)
}
