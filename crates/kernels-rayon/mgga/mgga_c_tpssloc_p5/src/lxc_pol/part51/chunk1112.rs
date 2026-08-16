//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1112/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1112(t24175: f64, t7687: f64, t6999: f64, t7940: f64, t532: f64, t7939: f64, t6879: f64, t12571: f64, t7025: f64, t23967: f64, t7432: f64, t7032: f64, t7435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26898 = t24175 * t7687;
    let t26902 = t7940 * t6999;
    let t26905 = t532 * t7939;
    let t26906 = t26905 * t6879;
    let t26911 = t12571 * t7025;
    let t26920 = t23967 * t7432;
    let t26936 = t7435 * t7032;
    (t26898, t26902, t26906, t26911, t26920, t26936)
}
