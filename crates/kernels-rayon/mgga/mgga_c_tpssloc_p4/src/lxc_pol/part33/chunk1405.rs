//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1405/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1405(t20356: f64, t6889: f64, t6890: f64, t80732: f64, t1843: f64, t20029: f64, t26366: f64, t28187: f64, t5321: f64, t568: f64, t6361: f64, t6440: f64, t7722: f64, t7750: f64, t81399: f64, t91531: f64, t91548: f64, t97732: f64, t97750: f64, t97756: f64) -> f64 {
    let t107484 = t80732 * t6889 * t6890 * t20356;
    let t107486 = -6.0_f64 * t20029 * t7750 + 3.0_f64 * t6361 * t7722 * t568 + 0.49348022005446793095e-1_f64 * t97732 - 0.78134368175290755733e-1_f64 * t91531 - 3.0_f64 * t5321 * t28187 - 0.57572692339687925277e-1_f64 * t97750 + 0.49348022005446793095e-1_f64 * t91548 - 6.0_f64 * t97756 * t1843 - t81399 + 6.0_f64 * t26366 * t6440 - 0.19739208802178717238e0_f64 * t107484;
    t107486
}
