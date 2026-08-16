//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1092/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1092(t4794: f64, t7: f64, t1448: f64, t448: f64, t34: f64, t38: f64, t4810: f64, t2620: f64, t5322: f64, t1532: f64, t2557: f64, t49: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19396 = t7 * t4794;
    let t19467 = t448 * t1448;
    let t19523 = t34 * t4794;
    let t19530 = t38 * t4810;
    let t19620 = t2620 * t5322;
    let t19623 = t2557 * t49 * t1532;
    (t19396, t19467, t19523, t19530, t19620, t19623)
}
