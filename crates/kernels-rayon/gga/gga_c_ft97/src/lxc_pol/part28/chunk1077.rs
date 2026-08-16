//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1077/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1077(t34678: f64, t8392: f64, t102689: f64, t102776: f64, t103073: f64, t103626: f64, t11490: f64, t11593: f64, t11810: f64, t11854: f64, t11863: f64, t137797: f64, t137804: f64, t144809: f64, t144846: f64, t145931: f64, t1901: f64, t1909: f64, t23327: f64, t23339: f64, t25919: f64, t26162: f64, t26171: f64, t26357: f64, t26367: f64, t26382: f64, t26390: f64, t3052: f64, t3219: f64, t32515: f64, t32597: f64, t32606: f64, t32635: f64, t3266: f64, t3271: f64, t34568: f64, t34627: f64, t379: f64, t46874: f64, t47443: f64, t59631: f64, t7229: f64, t8557: f64) -> f64 {
    let t145964 = t8392 * t34678;
    let t145991 = -2.0_f64 / 9.0_f64 * t1901 * t11863 * t144809 - t1901 * t8557 * t34627 * t379 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t145931 + 2.0_f64 / 3.0_f64 * t1901 * t46874 * t144846 - 2.0_f64 / 3.0_f64 * t1901 * t11810 * t32515 * t3266 - 2.0_f64 / 3.0_f64 * t1901 * t11490 * t137797 * t3271 + 2.0_f64 / 9.0_f64 * t1901 * t23327 * t26357 - 4.0_f64 / 3.0_f64 * t1901 * t59631 * t32606 - 2.0_f64 * t1901 * t26171 * t32597 * t3266 - 2.0_f64 / 3.0_f64 * t1901 * t11810 * t137804 * t3271 - 4.0_f64 / 3.0_f64 * t1901 * t103073 * t26162 - 4.0_f64 / 3.0_f64 * t1901 * t102776 * t26382 + 2.0_f64 / 27.0_f64 * t145964 + 4.0_f64 * t1901 * t103626 * t7229 * t3219 - 2.0_f64 / 9.0_f64 * t1901 * t11854 * t34568 * t379 - 2.0_f64 / 9.0_f64 * t1901 * t47443 * t32635 - 4.0_f64 / 9.0_f64 * t1901 * t102689 * t25919 + 2.0_f64 / 9.0_f64 * t11593 * t1909 * t32515 * t3052 - 4.0_f64 / 3.0_f64 * t1901 * t102776 * t26367 - 4.0_f64 / 3.0_f64 * t1901 * t11810 * t23339 * t26390;
    t145991
}
