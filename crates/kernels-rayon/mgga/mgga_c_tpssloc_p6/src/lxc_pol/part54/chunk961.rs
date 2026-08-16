//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 961/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk961(t461: f64, t6729: f64, t7324: f64, t2131: f64, t23508: f64, t1222: f64, t7334: f64, t2141: f64, t3540: f64, t3: f64, t1184: f64, t52: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24649 = t6729 * t461;
    let t24650 = t7324 * t24649;
    let t24658 = t2131 * t23508;
    let t24675 = t7334 * t1222;
    let t24681 = t2141 * t3540 / 6912.0_f64;
    let t24682 = t7324 * t3;
    let t24683 = t52 * t1184;
    (t24650, t24658, t24675, t24681, t24682, t24683)
}
