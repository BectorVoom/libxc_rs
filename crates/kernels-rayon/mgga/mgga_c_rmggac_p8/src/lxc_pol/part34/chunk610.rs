//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 610/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk610(t15064: f64, t15068: f64, t15076: f64, t15079: f64, t15082: f64, t2868: f64, t3188: f64, t3204: f64, t551: f64, t739: f64, t558: f64, t884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15431 = 0.38430329123504567781e-4_f64 * t15064;
    let t15432 = 0.17519306092901367187e-5_f64 * t15068;
    let t15433 = 0.44903406381989282115e-1_f64 * t15076;
    let t15434 = 0.14967802127329760705e-1_f64 * t15079;
    let t15435 = 0.76860658247009135557e-5_f64 * t15082;
    let t15437 = t2868 * t3188;
    let t15438 = 0.14967802127329760705e-1_f64 * t15437;
    let t15439 = t3204 * t551;
    let t15440 = t739 * t15439;
    let t15441 = 0.59871208509319042821e-1_f64 * t15440;
    let t15442 = t3204 * t558;
    let t15443 = t884 * t15442;
    (t15431, t15432, t15433, t15434, t15435, t15438, t15439, t15441, t15442, t15443)
}
