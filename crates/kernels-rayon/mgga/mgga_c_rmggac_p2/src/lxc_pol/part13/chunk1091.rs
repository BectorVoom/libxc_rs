//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1091/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1091(t41613: f64, t41619: f64, t302: f64, t36506: f64, t36508: f64, t36511: f64, t36513: f64, t36515: f64, t37964: f64, t41616: f64, t41627: f64, t41631: f64, t41635: f64, t41637: f64, t41639: f64, t41641: f64, t4965: f64, t72: f64, t9427: f64, t9595: f64) -> f64 {
    let t43761 = 0.60975299583150056624e-3_f64 * t41613;
    let t43763 = 0.60975299583150056624e-3_f64 * t41619;
    let t43780 = -t43761 - 0.1440846329149835838e-2_f64 * t41616 - t43763 + 2.0_f64 * t72 * t302 * t9595 + 0.212822999466489197e-4_f64 * t41627 - 0.5454932330849068346e-1_f64 * t41631 - 0.8182398496273602519e0_f64 * t41635 - 0.72732431077987577947e0_f64 * t41637 - 0.16364796992547205038e0_f64 * t41639 + 0.43639458646792546768e0_f64 * t41641 + 0.79828278012425390428e-1_f64 * t4965 * t9427 + t37964 + 0.19863479950205658386e-4_f64 * t36506 - 0.13242319966803772257e-3_f64 * t36508 + 0.39726959900411316772e-3_f64 * t36511 - 0.39726959900411316772e-3_f64 * t36513 - 0.13242319966803772257e-3_f64 * t36515;
    t43780
}
