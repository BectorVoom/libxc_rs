//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1370/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1370(t1692: f64, t1812: f64, t18728: f64, t18812: f64, t20021: f64, t20050: f64, t20065: f64, t20514: f64, t20526: f64, t21492: f64, t21499: f64, t2439: f64, t5849: f64, t5853: f64, t62610: f64, t6354: f64, t70805: f64, t70808: f64, t70816: f64, t70828: f64, t70839: f64, t70857: f64, t70887: f64, t70909: f64, t70960: f64) -> f64 {
    let t72460 = 3.0_f64 / 2.0_f64 * t2439 * t1812 * t70857 + 2.0_f64 * t20526 * t70805 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t70839 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t70909 + 3.0_f64 * t2439 * t6354 * t20021 - 3.0_f64 * t20526 * t70828 + 3.0_f64 / 2.0_f64 * t2439 * t5849 * t21499 - t1692 * t5853 * t70960 + t1692 * t18812 * t70808 - t1692 * t5853 * t70816 / 2.0_f64 - 3.0_f64 * t18728 * t70887 - t1692 * t20514 * t20050 - t1692 * t20514 * t20065 - 3.0_f64 * t62610 * t21492;
    t72460
}
