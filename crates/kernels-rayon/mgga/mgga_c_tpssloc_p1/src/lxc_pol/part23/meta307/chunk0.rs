//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1046/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1046(t1113: f64, t21749: f64, t136: f64, t11195: f64, t11204: f64, t14702: f64, t14766: f64, t18203: f64, t18219: f64, t18229: f64, t18494: f64, t18505: f64, t18512: f64, t21739: f64, t21741: f64, t21747: f64) -> (f64, f64, f64) {
    let t21750 = t1113 * t21749;
    let t21751 = t136 * t21750;
    let t21753 = -t11195 - 0.16431333333333333333e0_f64 * t18512 + 0.19931111111111111111e0_f64 * t18203 - 0.59793333333333333333e0_f64 * t18219 - 0.29896666666666666667e0_f64 * t18229 + 0.5477111111111111111e-1_f64 * t18494 - 0.32862666666666666666e0_f64 * t18505 - 0.28483875e1_f64 * t21739 + 0.46074375e0_f64 * t21741 - t11204 + 0.39862222222222222223e0_f64 * t14702 + 0.27385555555555555556e0_f64 * t14766 - 0.82156666666666666668e-1_f64 * t21747 + 0.49293999999999999999e0_f64 * t21751;
    (t21750, t21751, t21753)
}
