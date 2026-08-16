//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1431/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1431(t12626: f64, t1769: f64, t487: f64, t12627: f64, t1811: f64, t11239: f64, t1770: f64, t13061: f64, t13051: f64, t12909: f64, t17395: f64, t3781: f64, t5219: f64, t5330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56331 = t1769 * t12626;
    let t56332 = t56331 * t487;
    let t56393 = t12627 * t1811;
    let t56730 = t1770 * t11239;
    let t56731 = t56730 * t13061;
    let t57065 = t56730 * t13051;
    let t57147 = t12909 * t17395;
    let t57382 = t5219 * t3781 * t5330;
    (t56331, t56332, t56393, t56730, t56731, t57065, t57147, t57382)
}
