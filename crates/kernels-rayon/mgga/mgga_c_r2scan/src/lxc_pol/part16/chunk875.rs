//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 875/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk875(t2148: f64, t9296: f64, t6535: f64, t2139: f64, t2223: f64, t2614: f64, t2636: f64, t6241: f64, t7313: f64, t7608: f64, t7610: f64, t7618: f64, t7622: f64, t7627: f64, t7632: f64, t7925: f64, t7928: f64, t7939: f64, t8240: f64, t9280: f64, t9289: f64, t9294: f64) -> f64 {
    let t9297 = t2148 * t9296;
    let t9298 = t6535 * t9297;
    let t9300 = -t7608 + 0.2600466522016280569e0_f64 * t2139 * t9280 + t7610 - 0.21341733463216935736e0_f64 * t6241 + t7618 - t7622 + t7627 + t7632 + 0.17336443480108537126e0_f64 * t7313 * t2636 + 0.2600466522016280569e0_f64 * t8240 * t2614 + 0.16463622957338778997e0_f64 * t2223 * t9289 - 0.58218257753910989057e-2_f64 * t9294 + 0.11643651550782197811e-1_f64 * t9298 + t7925 + t7928 + t7939;
    t9300
}
