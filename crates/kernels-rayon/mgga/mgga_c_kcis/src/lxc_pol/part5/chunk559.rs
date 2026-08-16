//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 559/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk559(t2917: f64, t2966: f64, t961: f64) -> (f64, f64, f64, f64) {
    let t3013 = 0.40256666666666666667e0_f64 * t2917;
    let t3020 = 0.137975e0_f64 * t2966;
    let t3030 = t961 * t961;
    let t3031 = 1.0_f64 / t3030;
    (t3013, t3020, t3030, t3031)
}
