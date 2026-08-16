//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 860/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk860(t1437: f64, t8307: f64, t7440: f64, t8513: f64, t191: f64, t192: f64, t7681: f64, t3701: f64, t7752: f64, t4028: f64, t8326: f64, t7676: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33106 = t8307 * t1437;
    let t33114 = t8307 * t7440;
    let t33115 = t8513 * t33114;
    let t33133 = t7681 * t191 * t192;
    let t33136 = t3701 * t7752;
    let t33151 = t4028 * t8326;
    let t33152 = 2.0_f64 * t33151;
    let t33153 = t7676 * t8326;
    (t33106, t33115, t33133, t33136, t33151, t33152, t33153)
}
