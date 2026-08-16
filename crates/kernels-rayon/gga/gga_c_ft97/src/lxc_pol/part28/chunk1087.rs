//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1087/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1087(t1882: f64, t34649: f64, t34653: f64, t103: f64, t34482: f64, t34671: f64, t8392: f64, t34686: f64, t10969: f64, t110: f64, t11490: f64, t11593: f64, t11810: f64, t137739: f64, t138000: f64, t144958: f64, t145741: f64, t1871: f64, t1901: f64, t1902: f64, t23249: f64, t23323: f64, t26061: f64, t26145: f64, t26210: f64, t3052: f64, t32488: f64, t32527: f64, t32571: f64, t32620: f64, t3266: f64, t3271: f64, t34689: f64, t379: f64, t446: f64, t452: f64, t488: f64, t492: f64, t5710: f64, t5722: f64, t7229: f64, t83: f64, t8506: f64, t8557: f64, t925: f64) -> f64 {
    let t146604 = t1882 * t34649;
    let t146631 = t1882 * t34653;
    let t146637 = t103 * t34482;
    let t146642 = t8392 * t34671;
    let t146644 = t8392 * t34686;
    let t146671 = -t138000 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t146604 + 2.0_f64 / 3.0_f64 * t446 * t1871 * t110 * t144958 + t446 * t452 * t488 * t34482 * t492 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t8557 * t32571 * t925 - 2.0_f64 / 9.0_f64 * t1901 * t8557 * t32620 * t925 - 4.0_f64 / 9.0_f64 * t11593 * t8557 * t7229 * t3052 + 2.0_f64 / 3.0_f64 * t446 * t452 * t5710 * t26145 - 4.0_f64 / 9.0_f64 * t146631 - 2.0_f64 / 3.0_f64 * t446 * t452 * t10969 * t32527 + t1901 * t1902 * t146637 * t379 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t146642 + 2.0_f64 / 27.0_f64 * t146644 + 4.0_f64 / 3.0_f64 * t1901 * t11810 * t32488 * t3266 + 2.0_f64 * t1901 * t11490 * t137739 * t3271 - 4.0_f64 / 3.0_f64 * t1901 * t11490 * t23249 * t26145 + 4.0_f64 / 9.0_f64 * t11593 * t23323 * t26210 + t1901 * t8506 * t34689 / 9.0_f64 - t446 * t83 * t145741 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t452 * t26061 * t5722;
    t146671
}
