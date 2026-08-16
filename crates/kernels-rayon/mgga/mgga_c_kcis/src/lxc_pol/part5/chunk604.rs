//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 604/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk604(t3879: f64, t1331: f64, t659: f64, t1369: f64, t251: f64) -> (f64, f64, f64) {
    let t3880 = 0.13692777777777777778e0_f64 * t3879;
    let t3881 = t659 * t1331;
    let t3883 = t251 * t1369;
    (t3880, t3881, t3883)
}
