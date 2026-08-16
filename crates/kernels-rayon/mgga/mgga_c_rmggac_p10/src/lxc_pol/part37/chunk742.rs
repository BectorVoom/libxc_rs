//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 742/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk742(t14696: f64, t7345: f64, t14588: f64, t504: f64, t70018: f64, t70021: f64, t14469: f64, t2604: f64, t14589: f64, t7269: f64, t3219: f64, t7921: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71606 = t7345 * t14696;
    let t71607 = 0.15243824895787514157e-3_f64 * t71606;
    let t71608 = t504 * t14588;
    let t71619 = 0.50557909407869413937e0_f64 * t70018;
    let t71620 = 0.17347588262831798124e-3_f64 * t70021;
    let t71628 = t2604 * t14469;
    let t71630 = t14589 * t7269;
    let t71632 = t7921 * t3219;
    (t71607, t71608, t71619, t71620, t71628, t71630, t71632)
}
