//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1333/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1333(t1858: f64, t8153: f64, t2193: f64, t5363: f64, t30263: f64, t576: f64, t110020: f64, t110024: f64, t110032: f64, t110268: f64, t1396: f64, t1404: f64, t16546: f64, t2187: f64, t30218: f64, t3946: f64, t5364: f64, t5381: f64, t8154: f64, t8171: f64, t8241: f64) -> f64 {
    let t110899 = 2.0_f64 * t8153 * t1858;
    let t110904 = 2.0_f64 * t5363 * t2193;
    let t110910 = 2.0_f64 * t576 * t30263;
    let t110911 = 2.0_f64 * t1396 * t30263 + 2.0_f64 * t1404 * t30218 + t16546 * t2187 + t3946 * t8241 + 2.0_f64 * t5364 * t8171 + 2.0_f64 * t5381 * t8154 + 2.0_f64 * t110020 + t110024 + t110032 + t110268 + t110899 + t110904 + t110910;
    t110911
}
