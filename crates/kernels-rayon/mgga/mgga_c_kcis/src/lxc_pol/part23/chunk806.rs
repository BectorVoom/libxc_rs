//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 806/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk806(t1588: f64, t4413: f64, t12305: f64, t1628: f64, t4473: f64, t1625: f64, t4479: f64) -> (f64, f64, f64, f64) {
    let t12890 = t1588 * t4413;
    let t12915 = 0.51588271604938271604e-3_f64 * t12305;
    let t12930 = t4473 * t1628;
    let t12933 = t1625 * t4479;
    (t12890, t12915, t12930, t12933)
}
