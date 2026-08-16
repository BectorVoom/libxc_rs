//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1211/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1211(t38145: f64, t6093: f64, t9246: f64, t2201: f64, t3216: f64, t3319: f64, t3320: f64, t10698: f64, t12523: f64, t39958: f64, t39963: f64, t39965: f64, t39968: f64, t39969: f64, t39980: f64, t39983: f64, t39985: f64, t41641: f64) -> f64 {
    let t43447 = t6093 * t38145 * t9246;
    let t43451 = t2201 * t3319 * t3320 * t3216;
    let t43454 = t10698 * t12523;
    let t43457 = 0.13972381860938637374e0_f64 * t43447 - 0.23287303101564395623e-1_f64 * t43451 + 0.93149212406257582492e-1_f64 * t39958 + t39963 + 0.64025200389650807209e-1_f64 * t43454 + t39965 + t39968 + 0.14282990759302185292e-1_f64 * t39969 - t41641 - t39980 - t39983 + t39985;
    t43457
}
