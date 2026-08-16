//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 518/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk518(t136: f64, t555: f64, t2457: f64, t3964: f64, t4086: f64, t786: f64, t1432: f64, t1433: f64, t2470: f64, t3999: f64, t198: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4096 = t555 * t136;
    let t4099 = 0.11565819519348392139e-2_f64 * t3964 * t4096 * t2457;
    let t4100 = t4086 * t555;
    let t4101 = t786 * t4100;
    let t4113 = 0.13009920719177044025e-1_f64 * t1432 * t1433 * t2470;
    let t4114 = t3999 * t555;
    let t4139 = t198 * t531;
    (t4096, t4099, t4100, t4101, t4113, t4114, t4139)
}
