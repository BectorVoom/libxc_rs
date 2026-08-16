//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 581/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk581(t4565: f64, t4567: f64, t1662: f64, t3269: f64, t934: f64, t1045: f64, t3274: f64, t1103: f64, t347: f64, t1071: f64, t1646: f64) -> (f64, f64, f64, f64, f64) {
    let t4568 = t4565 * t4567;
    let t4572 = t3269 * t1662 * t934;
    let t4576 = t3274 * t1662 * t1045;
    let t4579 = t1103 * t347;
    let t4580 = t1071 * t1646;
    (t4568, t4572, t4576, t4579, t4580)
}
