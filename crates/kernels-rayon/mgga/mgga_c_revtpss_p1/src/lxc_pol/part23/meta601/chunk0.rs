//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2252/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2252(t12079: f64, t24078: f64, t1668: f64, t3302: f64, t357: f64, t19572: f64, t4982: f64, t6299: f64, t4893: f64, t12168: f64, t1651: f64, t19556: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24079 = t24078 * t12079;
    let t24083 = t3302 * t1668 * t357;
    let t24084 = t19572 * t24083;
    let t24089 = t4982 * t6299;
    let t24090 = t4893 * t24089;
    let t24093 = t24078 * t12168;
    let t24098 = t19556 * t1651;
    (t24079, t24083, t24084, t24089, t24090, t24093, t24098)
}
