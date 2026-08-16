//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1629/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1629(t14961: f64, t1559: f64, t23172: f64, t40314: f64, t40316: f64, t4514: f64, t51553: f64, t62843: f64, t62847: f64, t62874: f64, t62907: f64, t76127: f64, t77191: f64, t77197: f64, t820: f64) -> f64 {
    let t87850 = -0.26341796731742046395e1_f64 * t4514 * t76127 * t1559 + 0.13170898365871023197e0_f64 * t77191 + 0.21951497276451705328e-1_f64 * t77197 + 0.43902994552903410657e-1_f64 * t62843 - t40314 + t40316 - 0.39029762157531132076e-2_f64 * t62847 - 0.13878983423218070567e-1_f64 * t62874 + 0.15805078039045227836e2_f64 * t820 * t14961 * t23172 - 0.1040793657534163522e-1_f64 * t51553 + 0.13878983423218070567e-1_f64 * t62907;
    t87850
}
