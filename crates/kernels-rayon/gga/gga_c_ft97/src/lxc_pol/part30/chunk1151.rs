//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1151/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1151(t36161: f64, t8392: f64, t112680: f64, t112746: f64, t112888: f64, t112920: f64, t112987: f64, t114531: f64, t11593: f64, t143612: f64, t143653: f64, t143718: f64, t143720: f64, t143722: f64, t143753: f64, t1508: f64, t153550: f64, t15460: f64, t1901: f64, t24886: f64, t25271: f64, t28496: f64, t28516: f64, t28520: f64, t28524: f64, t2862: f64, t29056: f64, t29071: f64, t29154: f64, t29222: f64, t29399: f64, t296: f64, t34081: f64, t34082: f64, t4151: f64, t4162: f64, t4167: f64, t446: f64, t57089: f64, t72190: f64, t7629: f64, t99238: f64) -> f64 {
    let t153901 = t8392 * t36161;
    let t153922 = 4.0_f64 * t1901 * t114531 * t7629 * t4162 + 8.0_f64 / 3.0_f64 * t1901 * t72190 * t7629 * t4167 - 4.0_f64 / 9.0_f64 * t1901 * t112680 * t28516 - 2.0_f64 / 9.0_f64 * t1901 * t99238 * t29222 - 4.0_f64 / 9.0_f64 * t1901 * t112987 * t28520 + 4.0_f64 / 27.0_f64 * t1901 * t112746 * t28524 - 4.0_f64 / 3.0_f64 * t1901 * t112920 * t29056 + 2.0_f64 / 27.0_f64 * t143718 + 2.0_f64 / 27.0_f64 * t143720 - t143722 / 27.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t15460 * t25271 * t29399 - 4.0_f64 / 9.0_f64 * t11593 * t24886 * t29154 + t1901 * t143653 * t4151 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t153901 + 4.0_f64 / 3.0_f64 * t446 * t2862 * t1508 * t28496 - 2.0_f64 / 9.0_f64 * t1901 * t57089 * t34082 + 8.0_f64 * t1901 * t112888 * t34081 * t4162 + 2.0_f64 * t1901 * t29071 * t143612 * t4167 + 4.0_f64 / 3.0_f64 * t446 * t296 * t153550 - 4.0_f64 / 9.0_f64 * t143753;
    t153922
}
