//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 770/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk770(t24098: f64, t24164: f64, t533: f64, t1390: f64, t2095: f64, t23857: f64, t532: f64, t7216: f64, t6879: f64, t193: f64, t201: f64, t2056: f64) -> (f64, f64, f64, f64, f64) {
    let t24165 = t24098 + t24164;
    let t24166 = t533 * t24165;
    let t24167 = t24166 * t1390;
    let t24169 = t2095 * t23857;
    let t24175 = t532 * t7216;
    let t24176 = t24175 * t6879;
    let t24191 = t193 * t201 * t2056;
    (t24166, t24167, t24169, t24176, t24191)
}
