//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1374/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1374(t33: f64, t259: f64, t479: f64, t72412: f64, t1289: f64, t13335: f64, t1826: f64, t20632: f64, t21742: f64, t3431: f64, t4579: f64, t57: f64, t581: f64, t5889: f64, t6393: f64, t72460: f64, t72495: f64, t72531: f64, t72561: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t72564 = piecewise3(t480, 0.0_f64, t72412);
    let t72576 = piecewise3(t386, t72460 + t72495 + t72531 + t72561, t72564 * t57 / 2.0_f64 - t21742 * t581 / 2.0_f64 - t20632 * t1289 - t6393 * t3431 - t5889 * t4579 / 2.0_f64 - t1826 * t13335 / 2.0_f64);
    t72576
}
