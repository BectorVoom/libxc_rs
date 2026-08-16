//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 946/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk946(t1888: f64, t23270: f64, t2742: f64, t31332: f64, t232: f64, t6646: f64, t84842: f64, t112955: f64, t112959: f64, t112962: f64, t112967: f64, t112969: f64, t112973: f64, t112975: f64, t112980: f64, t2617: f64, t2679: f64, t31394: f64, t31395: f64, t812: f64) -> (f64, f64) {
    let t114632 = t1888 * t23270 * t31332 * t2742;
    let t114642 = t1888 * t6646 * t84842 * t232;
    let t114648 = -t112955 - t112959 + t112962 + t112967 - 0.82246703342411321825e-2_f64 * t114642 + t112969 + t112973 + t112975 + t112980 - t812 * t31394 * t2679 - 2.0_f64 * t2617 * t31395;
    (t114632, t114648)
}
