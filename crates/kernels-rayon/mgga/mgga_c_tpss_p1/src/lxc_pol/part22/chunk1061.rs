//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1061/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1061(t11584: f64, t2740: f64, t3944: f64, t8983: f64, t11562: f64, t11565: f64, t11572: f64, t11579: f64, t1461: f64, t3963: f64, t8450: f64, t8509: f64, t8514: f64, t8985: f64, t8989: f64, t8998: f64, t9004: f64) -> f64 {
    let t11586 = t2740 * t11584 / 3456.0_f64;
    let t11588 = t8983 * t3944;
    let t11590 = t2740 * t11588 / 3456.0_f64;
    let t11591 = t8985 / 3456.0_f64 + 11.0_f64 / 324.0_f64 * t8450 * t1461 - t11562 + t8998 / 864.0_f64 + t2740 * t11565 / 4608.0_f64 - t8509 * t11572 / 2304.0_f64 + t8514 * t11579 / 1152.0_f64 - t8989 * t3963 / 432.0_f64 + t11586 - t9004 / 3456.0_f64 + t11590;
    t11591
}
