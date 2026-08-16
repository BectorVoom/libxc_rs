//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1273/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1273(t1039: f64, t11470: f64, t12227: f64, t42294: f64, t42302: f64, t42304: f64, t42307: f64, t42310: f64, t42313: f64, t42316: f64, t42320: f64, t42326: f64, t42330: f64, t42334: f64, t42339: f64, t42344: f64, t42346: f64, t42349: f64, t860: f64) -> f64 {
    let t42364 = t1039 * t11470 + 2.0_f64 * t12227 * t860 - t42294 - t42302 + t42304 - t42307 - t42310 - t42313 - t42316 + t42320 - t42326 + t42330 - t42334 + t42339 - t42344 + t42346 - t42349;
    t42364
}
