//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1096/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1096(t1073: f64, t20045: f64, t2265: f64, t2266: f64, t3613: f64, t3621: f64, t39514: f64, t4454: f64, t4458: f64, t4872: f64, t4883: f64, t64926: f64, t64969: f64, t64985: f64, t75996: f64, t76199: f64, t76210: f64, t76267: f64, t85456: f64, t85465: f64, t85474: f64, t85491: f64, t8654: f64, t8680: f64, t920: f64) -> f64 {
    let t87906 = 10.0_f64 / 27.0_f64 * t64926 - 8.0_f64 / 3.0_f64 * t76199 - 20.0_f64 / 9.0_f64 * t64969 - 4.0_f64 / 3.0_f64 * t2265 * t2266 * t75996 * t920 - 12.0_f64 * t2265 * t8680 * t4458 * t4872 - 2.0_f64 / 3.0_f64 * t2265 * t8654 * t4454 * t4883 + 6.0_f64 * t2265 * t3621 * t85474 - 2.0_f64 * t2265 * t3613 * t85491 - 4.0_f64 / 3.0_f64 * t2265 * t2266 * t20045 * t1073 - 4.0_f64 / 3.0_f64 * t2265 * t3621 * t85456 + 2.0_f64 / 9.0_f64 * t2265 * t3613 * t85465 - 16.0_f64 * t2265 * t39514 * t76267 * t920 - 4.0_f64 / 9.0_f64 * t76210 - 40.0_f64 / 9.0_f64 * t64985;
    t87906
}
