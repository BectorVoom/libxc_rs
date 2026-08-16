//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1112/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1112(t2684: f64, t7354: f64, t9829: f64, t1391: f64, t9833: f64, t15490: f64, t7584: f64, t9438: f64, t21456: f64, t2365: f64, t7390: f64, t7416: f64, t9834: f64) -> (f64, f64, f64, f64, f64) {
    let t28987 = t2684 * t7354 * t9829;
    let t28990 = t2684 * t1391 * t9833;
    let t29001 = t7584 * t9438 * t15490;
    let t29009 = 0.59584149919750711116e-1_f64 * t7390 * t2365 * t21456;
    let t29011 = 0.17041300423964777634e0_f64 * t7416 * t9834;
    (t28987, t28990, t29001, t29009, t29011)
}
