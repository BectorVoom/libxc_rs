//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1017/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1017(t14066: f64, t3564: f64, t189: f64, t4579: f64, t581: f64, t1364: f64, t821: f64) -> (f64, f64, f64) {
    let t14068 = 24.0_f64 * t3564 * t14066;
    let t14069 = t189 * t4579;
    let t14070 = t14069 * t581;
    let t14072 = 12.0_f64 * t3564 * t14070;
    let t14076 = t1364 * t821;
    (t14068, t14072, t14076)
}
