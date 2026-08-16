//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 871/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk871(t1229: f64, t3242: f64, t11153: f64, t3584: f64, t1734: f64, t3508: f64, t1089: f64, t475: f64, t1744: f64, t3540: f64, t1731: f64, t1706: f64, t3545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15615 = t1229 * t3242;
    let t15654 = t3584 * t11153;
    let t15659 = t1734 * t3508;
    let t15701 = t475 * t1089;
    let t15717 = t1744 * t3540;
    let t15719 = t1731 * t3540;
    let t15727 = t1706 * t3545;
    (t15615, t15654, t15659, t15701, t15717, t15719, t15727)
}
