//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2192/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2192(t1307: f64, t1842: f64, t22635: f64, t26331: f64, t26337: f64, t26189: f64, t26193: f64, t6888: f64, t22892: f64, t7691: f64, t90544: f64, t1835: f64, t254: f64) -> (f64, f64, f64, f64) {
    let t97721 = t1842 * t1307;
    let t97724 = t26331 * t22635 * t26337 * t97721;
    let t97729 = t6888 * t26193 * t26189;
    let t97732 = t22892 * t90544 * t7691;
    let t97740 = t1835 * t254;
    (t97724, t97729, t97732, t97740)
}
