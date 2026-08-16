//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 741/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk741(t1396: f64, t5627: f64, t1468: f64, t1464: f64, t1928: f64, t556: f64) -> (f64, f64, f64, f64) {
    let t5628 = t1396 * t5627;
    let t5629 = t1468 * t5628;
    let t5630 = t1464 * t5629;
    let t5632 = t1928 * t556;
    (t5628, t5629, t5630, t5632)
}
