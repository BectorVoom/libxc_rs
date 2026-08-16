//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1418/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1418(t119737: f64, t119743: f64, t119766: f64, t121275: f64, t121779: f64, t121861: f64, t1649: f64, t1877: f64, t24191: f64, t24339: f64, t2522: f64, t25927: f64, t25938: f64, t26756: f64, t31430: f64, t33065: f64, t33476: f64, t33531: f64, t7114: f64, t83555: f64, t84797: f64, t8566: f64, t8586: f64, t92276: f64) -> f64 {
    let t122072 = -t1877 * t7114 * t119766 / 2.0_f64 - t1877 * t7114 * t119737 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t84797 * t33531 + t26756 * t119743 - t1877 * t24339 * t33065 / 2.0_f64 + t1877 * t31430 * t1649 / 2.0_f64 + t26756 * t25927 * t121779 - t1877 * t92276 * t8586 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t25938 - 3.0_f64 / 2.0_f64 * t24191 * t83555 * t33476 + 3.0_f64 * t24191 * t25927 * t121275 - t121861;
    t122072
}
