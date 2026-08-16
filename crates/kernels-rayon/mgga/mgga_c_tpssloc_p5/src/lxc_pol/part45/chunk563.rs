//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 563/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk563(t240: f64, t6612: f64, t812: f64, t831: f64, t1899: f64, t838: f64, t234: f64, t59: f64, t849: f64, t6580: f64, t6582: f64, t6587: f64, t6594: f64, t6603: f64, t6607: f64, t6610: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6613 = t6612 * t240;
    let t6614 = t812 * t6613;
    let t6615 = t6614 * t831;
    let t6617 = t1899 * t838;
    let t6618 = 7.0_f64 / 2304.0_f64 * t6617;
    let t6619 = t234 * t59;
    let t6620 = t6619 * t240;
    let t6621 = t812 * t6620;
    let t6622 = t6621 * t849;
    let t6624 = -t6580 - t6582 / 48.0_f64 - t6587 - 0.12111826828242117256e-2_f64 * t6594 - t6603 - 0.20186378047070195427e-3_f64 * t6607 + t6610 / 1536.0_f64 - t6615 / 1536.0_f64 - t6618 - t6622 / 384.0_f64;
    (t6613, t6614, t6615, t6617, t6619, t6620, t6621, t6622, t6624)
}
