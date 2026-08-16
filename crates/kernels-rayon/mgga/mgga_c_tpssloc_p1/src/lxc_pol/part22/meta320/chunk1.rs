//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1504/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1504(t1089: f64, t475: f64, t607: f64, t1744: f64, t3540: f64, t1731: f64, t1222: f64, t4961: f64, t1706: f64, t3545: f64, t11818: f64, t1735: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15701 = t475 * t1089;
    let t15702 = t15701 * t607;
    let t15717 = t1744 * t3540;
    let t15719 = t1731 * t3540;
    let t15722 = t4961 * t1222 / 432.0_f64;
    let t15727 = t1706 * t3545;
    let t15730 = t248 * t11818 * t1735;
    (t15701, t15702, t15717, t15719, t15722, t15727, t15730)
}
