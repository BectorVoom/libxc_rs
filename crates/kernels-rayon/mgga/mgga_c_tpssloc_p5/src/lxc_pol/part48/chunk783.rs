//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 783/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk783(t28: f64, t265: f64, t504: f64, t24379: f64, t2071: f64, t2250: f64, t24419: f64, t52: f64, t607: f64, t7150: f64, t24387: f64, t2094: f64, t3701: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t24420 = piecewise3(t505, 0.0_f64, t24379);
    let t24427 = piecewise3(t401, t24419, t24420 * t52 / 2.0_f64 - t7150 * t607 - t2071 * t2250 / 2.0_f64);
    let t24428 = t24387 + t24427;
    let t24432 = t2094 * t3701;
    (t24428, t24432)
}
