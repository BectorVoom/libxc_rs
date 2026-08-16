//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1228/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1228(t1011: f64, t1022: f64, t360: f64, t23478: f64, t6785: f64, t2770: f64, t381: f64, t254: f64, t382: f64, t10164: f64, t1955: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25659 = t1011 * t1022;
    let t25660 = t25659 * t360;
    let t25713 = t23478 * t6785;
    let t25721 = t381 * t2770;
    let t25757 = t382 * t254;
    let t25758 = t10164 * t1955;
    let t25796 = t343 * t381;
    (t25660, t25713, t25721, t25757, t25758, t25796)
}
