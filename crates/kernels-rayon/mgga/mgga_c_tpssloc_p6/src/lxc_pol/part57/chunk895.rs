//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 895/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk895(t33620: f64, t652: f64, t2095: f64, t33136: f64, t1983: f64, t1774: f64, t1869: f64, t2036: f64, t2075: f64, t33579: f64, t33601: f64, t33605: f64, t33611: f64, t33615: f64, t33619: f64, t510: f64, t574: f64, t7451: f64, t7670: f64, t7890: f64, t7904: f64, t7943: f64, t8450: f64, t8519: f64) -> (f64, f64) {
    let t33622 = 2.0_f64 * t652 * t33620;
    let t33623 = t2095 * t33136;
    let t33624 = t1983 * t33623;
    let t33625 = -t1774 * t8519 - t1869 * t7890 - t2036 * t7670 - t2075 * t7451 - t33579 * t510 + t33601 * t574 + 3.0_f64 * t7904 * t8450 - t7943 * t8450 + t33605 - t33611 + t33615 - t33619 - t33622 - t33624;
    (t33623, t33625)
}
