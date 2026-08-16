//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3929/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3929(t1507: f64, t2357: f64, t10227: f64, t10241: f64, t105: f64, t13493: f64, t13497: f64, t13500: f64, t13506: f64, t21835: f64, t21845: f64, t21846: f64, t21850: f64, t21860: f64, t2255: f64, t2256: f64, t2349: f64, t2350: f64, t2358: f64, t2362: f64, t31283: f64, t31443: f64, t4269: f64, t4279: f64, t46212: f64, t49777: f64, t49787: f64, t49804: f64, t580: f64, t5823: f64, t5907: f64, t5911: f64, t656: f64, t658: f64, t97: f64) -> f64 {
    let t75625 = t1507 * t2357;
    let t75634 = -10.0_f64 / 27.0_f64 * t97 * t21835 * t2256 + 20.0_f64 / 9.0_f64 * t97 * t4269 * t580 - 10.0_f64 / 27.0_f64 * t97 * t10227 * t5823 * t2350 + 100.0_f64 / 81.0_f64 * t1507 * t13493 - 50.0_f64 / 3.0_f64 * t1507 * t13506 + 40.0_f64 / 81.0_f64 * t105 * t46212 * t5907 * t2358 - 20.0_f64 / 9.0_f64 * t105 * t4279 * t580 - 10.0_f64 / 27.0_f64 * t105 * t10241 * t5911 * t2358 - 100.0_f64 / 27.0_f64 * t656 * t21846 + 20.0_f64 / 9.0_f64 * t97 * t2349 * t21850 * t658 + 10.0_f64 / 9.0_f64 * t97 * t21845 * t2256 - 100.0_f64 / 27.0_f64 * t1507 * t13500 - 10.0_f64 / 27.0_f64 * t105 * t21860 * t2362 + 200.0_f64 / 27.0_f64 * t75625 * t13497 + t49804 - 40.0_f64 / 27.0_f64 * t49777 * t31283 * t2255 + 40.0_f64 / 27.0_f64 * t49787 * t31443 * t2255;
    t75634
}
