//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 821/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk821(t10151: f64, t2464: f64, t2465: f64, t2487: f64, t10417: f64, t1415: f64, t7030: f64, t12960: f64, t31051: f64, t41588: f64, t41592: f64, t41595: f64, t41600: f64, t41604: f64, t41607: f64, t41610: f64, t41613: f64, t41616: f64, t41619: f64, t41621: f64, t41624: f64, t41627: f64, t41630: f64, t41631: f64, t41636: f64) -> f64 {
    let t41640 = t2487 * t2464 * t2465 * t10151;
    let t41643 = t1415 * t10417 * t7030;
    let t41645 = t31051 * t12960;
    let t41646 = 0.19171462976960374838e1_f64 * t41645;
    let t41647 = 0.19171462976960374838e1_f64 * t41588 - 0.11502877786176224903e1_f64 * t41592 - t41595 + t41600 - t41604 - t41607 - t41610 + t41613 + t41616 - t41619 + 0.59584149919750711116e-1_f64 * t41621 + t41624 + t41627 + t41630 + 0.38342925953920749676e0_f64 * t41631 + 0.38342925953920749676e0_f64 * t41636 - 0.85206502119823888169e-1_f64 * t41640 - 0.29792074959875355558e-1_f64 * t41643 + t41646;
    t41647
}
