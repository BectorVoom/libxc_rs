//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1190/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1190(t12652: f64, t4972: f64, t4582: f64, t11153: f64, t3584: f64, t14165: f64, t1734: f64, t3508: f64, t1089: f64, t1215: f64, t607: f64, t3578: f64) -> (f64, f64, f64) {
    let t15649 = t4972 * t12652;
    let t15650 = t4582 * t15649;
    let t15654 = t3584 * t11153;
    let t15655 = t15654 * t14165;
    let t15656 = t4582 * t15655;
    let t15659 = t1734 * t3508;
    let t15660 = t1215 * t1089;
    let t15661 = t15660 * t607;
    let t15662 = t15659 * t15661;
    let t15663 = t3578 * t15662;
    (t15650, t15656, t15663)
}
