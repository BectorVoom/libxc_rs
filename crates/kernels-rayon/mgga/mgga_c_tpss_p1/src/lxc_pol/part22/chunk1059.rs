//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1059/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1059(t11031: f64, t3923: f64, t11035: f64, t242: f64, t2751: f64, t3758: f64, t967: f64, t10984: f64, t970: f64, t11524: f64, t11528: f64, t11529: f64, t11532: f64, t11536: f64, t2685: f64, t2748: f64, t3920: f64, t3983: f64, t925: f64) -> f64 {
    let t11539 = t3923 * t11031;
    let t11542 = t3923 * t11035;
    let t11548 = t242 * t2751 * t3758;
    let t11550 = t967 * t11548 / 3456.0_f64;
    let t11552 = t242 * t970 * t10984;
    let t11555 = -2.0_f64 / 81.0_f64 * t2685 * t3920 - t11524 + t11528 + t925 * t11529 / 108.0_f64 + t925 * t11532 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t925 * t11536 - t925 * t11539 / 72.0_f64 - t925 * t11542 / 144.0_f64 - t2748 * t3983 / 432.0_f64 + t11550 + t967 * t11552 / 4608.0_f64;
    t11555
}
