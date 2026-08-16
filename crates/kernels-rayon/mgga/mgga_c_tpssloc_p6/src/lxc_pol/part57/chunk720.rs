//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 720/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk720(t7484: f64, t794: f64, t6562: f64, t1887: f64, t23056: f64, t6547: f64, t7485: f64, t1484: f64, t22690: f64, t841: f64, t23122: f64, t4166: f64, t6620: f64) -> (f64, f64, f64, f64, f64) {
    let t25035 = t794 * t7484;
    let t25036 = t6562 * t25035;
    let t25038 = t23056 * t1887;
    let t25049 = t6547 * t7485;
    let t25064 = t22690 * t841 * t1484;
    let t25065 = t23122 * t25064;
    let t25068 = t4166 * t6620;
    (t25036, t25038, t25049, t25065, t25068)
}
