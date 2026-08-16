//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1103/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1103(t29059: f64, t7754: f64, t3338: f64, t6555: f64, t18463: f64, t389: f64, t1813: f64, t5026: f64, t1817: f64, t4999: f64, t3227: f64, t6717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29060 = t7754 * t29059;
    let t29062 = t3338 * t6555;
    let t29063 = t7754 * t29062;
    let t29065 = t18463 * t389;
    let t29067 = t5026 * t1813;
    let t29069 = t4999 * t1817;
    let t29071 = t3227 * t6717;
    (t29060, t29062, t29063, t29065, t29067, t29069, t29071)
}
