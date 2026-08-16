//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1366/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1366(t1570: f64, t3689: f64, t11977: f64, t11983: f64, t12014: f64, t1340: f64, t1344: f64, t2268: f64, t30009: f64, t30014: f64, t30049: f64, t31737: f64, t31755: f64, t31758: f64, t31760: f64, t31766: f64, t31772: f64, t31777: f64, t3808: f64, t6313: f64) -> (f64, f64) {
    let t38362 = t1570 * t3689;
    let t38368 = -t30009 - t30014 - t31737 - 0.63233348079280332442e-2_f64 * t3808 * t12014 - 0.19918504644973304719e0_f64 * t2268 * t11977 * t1344 + 0.34146007962811379518e0_f64 * t2268 * t38362 * t1340 + 0.15176003539027279786e0_f64 * t6313 * t11983 - t30049 + t31755 - t31758 - t31760 + t31766 - t31772 + t31777;
    (t38362, t38368)
}
