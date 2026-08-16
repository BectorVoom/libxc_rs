//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1381/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1381(t28: f64, t2161: f64, t2250: f64, t24916: f64, t52: f64, t607: f64, t7402: f64, t83655: f64, t86534: f64, t9258: f64, t113: f64, t12507: f64, t1393: f64, t2165: f64, t24924: f64, t24939: f64, t574: f64, t652: f64, t671: f64, t7266: f64, t83882: f64, t83884: f64, t83888: f64, t83896: f64, t83905: f64, t83913: f64, t83917: f64, t83919: f64, t83921: f64, t83924: f64, t83928: f64, t83932: f64, t83939: f64, t85613: f64, t85627: f64, t9347: f64, t9416: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t86544 = piecewise3(t401, t83655, t86534 * t52 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24916 * t607 - 3.0_f64 / 2.0_f64 * t7402 * t2250 - t2161 * t9258 / 2.0_f64);
    let t86548 = -6.0_f64 * t7266 * t12507 - 2.0_f64 * t652 * t2165 * t9416 - 6.0_f64 * t652 * t24924 * t671 + t83882 + t83884 - t83888 - t83896 + t85613 * t574 + 3.0_f64 * t24939 * t1393 + t83905 - t83913 - t83917 - t83919 - t83921 - t83924 - t83928 + t83932 - t83939 - t113 * (t85627 + t86544) - t9347 * t2165;
    t86548
}
