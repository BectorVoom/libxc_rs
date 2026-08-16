//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1482/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1482(t1229: f64, t204: f64, t1090: f64, t1227: f64, t248: f64, t11692: f64, t1174: f64, t1177: f64, t11779: f64, t11781: f64, t11825: f64, t1213: f64, t1214: f64, t1216: f64, t3490: f64, t3515: f64, t3527: f64, t3578: f64, t3585: f64, t43719: f64, t43752: f64, t43792: f64, t43796: f64, t44668: f64, t44798: f64, t45250: f64, t45251: f64, t45256: f64, t45260: f64, t45262: f64, t45264: f64, t45266: f64, t45271: f64, t45283: f64, t475: f64) -> f64 {
    let t45293 = t204 * t1229;
    let t45296 = t1227 * t248 * t45293 * t1090;
    let t45311 = -t45250 + t11692 * t3578 * t1216 * t45251 / 384.0_f64 + 5.0_f64 / 1728.0_f64 * t45256 + 5.0_f64 / 864.0_f64 * t45260 + t45262 / 384.0_f64 - t45264 / 576.0_f64 - t45266 / 1152.0_f64 - 5.0_f64 / 1944.0_f64 * t45271 + 5.0_f64 / 4608.0_f64 * t1227 * t248 * t3585 * t43752 + 5.0_f64 / 384.0_f64 * t1227 * t248 * t3585 * t43796 - t45283 / 192.0_f64 - t1174 * t1177 * t43719 / 8.0_f64 - t3515 * t248 * t1214 * t44668 * t475 / 1024.0_f64 - t45296 / 3888.0_f64 - 5.0_f64 / 1296.0_f64 * t3490 * t11781 - 5.0_f64 / 432.0_f64 * t1227 * t248 * t11779 * t43792 + t1213 * t248 * t1214 * t44798 * t475 / 3072.0_f64 - t11825 * t3527 / 768.0_f64;
    t45311
}
