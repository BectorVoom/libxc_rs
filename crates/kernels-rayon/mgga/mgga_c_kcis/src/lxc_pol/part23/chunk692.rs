//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 692/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk692(t2257: f64, t7974: f64, t1592: f64, t251: f64, t1598: f64) -> (f64, f64, f64) {
    let t7976 = 0.11584201388888888889e-3_f64 * t2257 * t7974;
    let t7977 = t1592 * t251;
    let t7978 = t7977 * t1598;
    (t7976, t7977, t7978)
}
