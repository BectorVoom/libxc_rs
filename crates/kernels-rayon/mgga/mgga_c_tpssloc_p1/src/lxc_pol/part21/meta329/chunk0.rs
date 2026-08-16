//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1703/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1703(t10022: f64, t248: f64, t557: f64, t555: f64, t10027: f64, t541: f64, t12267: f64, t1362: f64, t3777: f64, t3865: f64) -> (f64, f64, f64, f64, f64) {
    let t12328 = t10022 * t557 * t248;
    let t12330 = 595.0_f64 / 10368.0_f64 * t555 * t12328;
    let t12335 = 455.0_f64 / 1296.0_f64 * t10027 * t541;
    let t12336 = t12267 * t1362;
    let t12339 = t3777 * t3865;
    (t12328, t12330, t12335, t12336, t12339)
}
