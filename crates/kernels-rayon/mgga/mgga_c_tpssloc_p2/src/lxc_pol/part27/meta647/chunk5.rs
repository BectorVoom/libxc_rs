//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2235/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2235(t1011: f64, t4649: f64, t10474: f64, t381: f64, t82514: f64, t1615: f64, t3032: f64, t25483: f64, t23384: f64, t25456: f64, t1049: f64, t11065: f64, t13980: f64, t13985: f64, t14590: f64, t23346: f64, t23601: f64, t23602: f64, t25459: f64, t25484: f64, t25485: f64, t25486: f64, t25487: f64, t25516: f64, t25714: f64, t2780: f64, t3127: f64, t3132: f64, t4594: f64, t6687: f64, t6784: f64, t7619: f64, t82513: f64, t82534: f64, t82694: f64) -> (f64, f64, f64) {
    let t89194 = t4649 * t1011;
    let t89204 = t82514 * t10474 * t381;
    let t89205 = t1615 * t3032;
    let t89210 = t82514 * t25483;
    let t89224 = 0.54831135561607547884e-2_f64 * t23384 * t25456;
    let t89225 = -0.87729816898572076613e-1_f64 * t82534 * t25487 + 0.27415567780803773942e-2_f64 * t6687 * t6784 * t25516 * t2780 + 0.3289868133696452873e-1_f64 * t23601 * t23602 * t3127 * t1049 * t25486 + 0.3289868133696452873e-1_f64 * t23601 * t25484 * t89194 * t4594 + 0.16449340668482264365e-1_f64 * t23601 * t25484 * t25485 * t13980 + 0.49348022005446793095e-1_f64 * t82513 * t89204 * t89205 * t13985 - 0.49348022005446793095e-1_f64 * t82513 * t89210 * t89205 * t3132 - 6.0_f64 * t11065 * t7619 * t14590 - 0.14621636149762012769e-1_f64 * t82694 + 0.43864908449286038306e-1_f64 * t23346 * t25714 + 0.43864908449286038306e-1_f64 * t23346 * t25459 - t89224;
    (t89194, t89205, t89225)
}
