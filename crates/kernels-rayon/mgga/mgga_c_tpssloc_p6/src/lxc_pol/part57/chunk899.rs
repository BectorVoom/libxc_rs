//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 899/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk899(t111: f64, t6470: f64, t2239: f64, t5385: f64, t1887: f64, t22797: f64, t268: f64, t547: f64, t6559: f64, t225: f64, t22643: f64, t23069: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55388 = t6470 * t111;
    let t55921 = t5385 * t2239;
    let t81159 = t22797 * t1887;
    let t81228 = t6559 * t547 * t268;
    let t81326 = t22643 * t225;
    let t81591 = t23069 * t1887;
    (t55388, t55921, t81159, t81228, t81326, t81591)
}
