//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2648/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2648(t157: f64, t73989: f64, t74009: f64, t182: f64, t20675: f64, t3701: f64, t39305: f64, t1388: f64, t20077: f64, t20681: f64, t3918: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t5160: f64, t5187: f64, t53783: f64, t53788: f64, t53797: f64, t55224: f64, t73958: f64, t73959: f64, t73960: f64, t73961: f64, t73962: f64, t73968: f64, t73969: f64) -> (f64, f64, f64, f64) {
    let t74011 = (t73989 + t74009) * t157;
    let t74013 = 0.19751673498613801407e-1_f64 * t74011 * t182;
    let t74014 = t20675 * t3701;
    let t74017 = 0.10389515463408878255e3_f64 * t39305;
    let t74020 = -t1388 * t5160 * t74014 - 9.0_f64 * t20077 * t3918 * t5187 + 18.0_f64 * t20681 * t55224 - t39249 - t39256 - t39261 - t39266 - t39304 + t53783 + t53788 + t53797 - t73958 - t73959 - t73960 - t73961 - t73962 - t73968 - t73969 + t74013 + t74017;
    (t74011, t74013, t74017, t74020)
}
