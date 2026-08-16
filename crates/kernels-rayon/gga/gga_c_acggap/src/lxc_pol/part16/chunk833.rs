//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 833/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk833(t1896: f64, t2001: f64, t1901: f64, t1734: f64, t599: f64, t142: f64, t2030: f64, t1795: f64, t604: f64, t2060: f64, t7674: f64, t7678: f64, t7697: f64, t8835: f64, t8862: f64, t9313: f64, t9316: f64, t9318: f64, t9320: f64, t9328: f64, t9329: f64, t9331: f64, t9682: f64, t9688: f64, t9692: f64, t9694: f64) -> (f64, f64, f64, f64, f64) {
    let t9696 = t2001 * t1896;
    let t9698 = t2001 * t1901;
    let t9700 = t599 * t1734;
    let t9701 = t142 * t9700;
    let t9702 = t2030 * t9701;
    let t9704 = t604 * t1795;
    let t9705 = t142 * t9704;
    let t9706 = t2060 * t9705;
    let t9709 = -0.53592522647587171215e-3_f64 * t9682 - t7674 + 0.40015750243531754508e-2_f64 * t8835 + t9313 - t9316 + t9318 + t9320 + 0.10718504529517434243e-3_f64 * t9688 - 0.31448092289604152068e-3_f64 * t9692 - 0.17149607247227894789e-2_f64 * t9694 + 0.85748036236139473944e-3_f64 * t9696 - 0.85748036236139473944e-3_f64 * t9698 + 0.114609375e-1_f64 * t9702 + 0.7640625e-2_f64 * t9706 + 0.37737710747524982482e-2_f64 * t8862 - t9328 - t9329 + t7678 - t7697 - t9331;
    (t9700, t9701, t9704, t9705, t9709)
}
