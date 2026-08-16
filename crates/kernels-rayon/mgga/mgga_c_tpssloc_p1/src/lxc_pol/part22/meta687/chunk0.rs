//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2264/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2264(t2403: f64, t6014: f64, t6017: f64, t18502: f64, t699: f64, t18499: f64, t18509: f64, t18507: f64, t3356: f64, t6031: f64, t1128: f64, t18668: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63893 = t2403 * t6014;
    let t63911 = t2403 * t6017;
    let t64074 = t699 * t18502;
    let t64076 = t699 * t18499;
    let t64087 = t699 * t18509;
    let t64089 = t699 * t18507;
    let t64103 = t6031 * t3356;
    let t64254 = t18668 * t1128;
    (t63893, t63911, t64074, t64076, t64087, t64089, t64103, t64254)
}
