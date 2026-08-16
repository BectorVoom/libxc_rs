//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 442/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk442(t2917: f64, t242: f64, t1060: f64, t250: f64, t253: f64, t659: f64, t946: f64, t251: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2947 = 4.0_f64 / 9.0_f64 * t2917;
    let t2955 = 0.39862222222222222223e0_f64 * t2917;
    let t2960 = 1.0_f64/f64::sqrt(t242);
    let t2966 = t250 * t1060 * t253;
    let t2967 = 0.13692777777777777778e0_f64 * t2966;
    let t2968 = t659 * t946;
    let t2970 = t251 * t992;
    (t2947, t2955, t2960, t2966, t2967, t2968, t2970)
}
