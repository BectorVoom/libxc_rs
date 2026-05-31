//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 833/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk833<F: Float>(t1896: F, t2001: F, t1901: F, t1734: F, t599: F, t142: F, t2030: F, t1795: F, t604: F, t2060: F, t7674: F, t7678: F, t7697: F, t8835: F, t8862: F, t9313: F, t9316: F, t9318: F, t9320: F, t9328: F, t9329: F, t9331: F, t9682: F, t9688: F, t9692: F, t9694: F) -> (F, F, F, F, F) {
    let t9696 = t2001 * t1896;
    let t9698 = t2001 * t1901;
    let t9700 = t599 * t1734;
    let t9701 = t142 * t9700;
    let t9702 = t2030 * t9701;
    let t9704 = t604 * t1795;
    let t9705 = t142 * t9704;
    let t9706 = t2060 * t9705;
    let t9709 = -F::cast_from(0.53592522647587171215e-3_f64) * t9682 - t7674 + F::cast_from(0.40015750243531754508e-2_f64) * t8835 + t9313 - t9316 + t9318 + t9320 + F::cast_from(0.10718504529517434243e-3_f64) * t9688 - F::cast_from(0.31448092289604152068e-3_f64) * t9692 - F::cast_from(0.17149607247227894789e-2_f64) * t9694 + F::cast_from(0.85748036236139473944e-3_f64) * t9696 - F::cast_from(0.85748036236139473944e-3_f64) * t9698 + F::cast_from(0.114609375e-1_f64) * t9702 + F::cast_from(0.7640625e-2_f64) * t9706 + F::cast_from(0.37737710747524982482e-2_f64) * t8862 - t9328 - t9329 + t7678 - t7697 - t9331;
    (t9700, t9701, t9704, t9705, t9709)
}
