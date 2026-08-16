//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1319/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1319(t33155: f64, t7584: f64, t7585: f64, t10848: f64, t22748: f64, t32356: f64, t701: f64, t20157: f64, t323: f64, t32349: f64, t320: f64, t32608: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33624 = 0.11502877786176224903e2_f64 * t7584 * t7585 * t33155;
    let t33626 = 0.23005755572352449806e2_f64 * t22748 * t10848;
    let t33627 = t32356 * t701;
    let t33630 = 0.23005755572352449806e2_f64 * t7584 * t7585 * t33627;
    let t33633 = 0.40899121017515466321e1_f64 * t323 * t20157 * t32349;
    let t33637 = 0.19427082483319846503e2_f64 * t320 * t831 * t20157 * t32608;
    (t33624, t33626, t33627, t33630, t33633, t33637)
}
