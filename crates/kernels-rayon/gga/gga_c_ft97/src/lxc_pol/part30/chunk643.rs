//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 643/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk643(t3746: f64, t6161: f64, t2606: f64, t3837: f64, t13885: f64, t24668: f64, t3842: f64, t14127: f64, t11593: f64, t1901: f64, t24742: f64, t24757: f64, t28326: f64, t28330: f64, t28334: f64, t28338: f64, t28341: f64, t28346: f64, t28350: f64, t28353: f64, t28357: f64, t446: f64) -> (f64, f64, f64, f64) {
    let t28360 = t6161 * t3746;
    let t28361 = t2606 * t28360;
    let t28364 = t6161 * t3837;
    let t28365 = t13885 * t28364;
    let t28368 = t24668 * t3842;
    let t28369 = t14127 * t28368;
    let t28372 = 2.0_f64 / 3.0_f64 * t446 * t28326 - t446 * t28330 / 9.0_f64 + t446 * t28334 / 3.0_f64 - t24742 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t28338 - t24757 - 2.0_f64 / 9.0_f64 * t1901 * t28341 + 2.0_f64 / 27.0_f64 * t1901 * t28346 - t1901 * t28350 / 9.0_f64 - t28353 / 27.0_f64 + t1901 * t28357 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t11593 * t28361 - 2.0_f64 / 3.0_f64 * t1901 * t28365 - 2.0_f64 / 3.0_f64 * t1901 * t28369;
    (t28360, t28364, t28368, t28372)
}
