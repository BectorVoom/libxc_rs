//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2272/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2272(t5087: f64, t6449: f64, t12254: f64, t24228: f64, t141: f64, t1145: f64, t24244: f64, t16706: f64, t16876: f64, t20276: f64, t20278: f64, t20280: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24265: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24267 = t5087 * t6449;
    let t24271 = t12254 * t24228;
    let t24272 = t141 * t24271;
    let t24274 = t1145 * t24244;
    let t24275 = t141 * t24274;
    let t24285 = -0.28483875e1_f64 * t24265 + 0.46074375e0_f64 * t24267 + 0.39862222222222222223e0_f64 * t16706 + 0.27385555555555555556e0_f64 * t16876 + 0.36514074074074074075e-1_f64 * t24272 + 0.49293999999999999999e0_f64 * t24275 + 0.5477111111111111111e-1_f64 * t20276 - 0.32862666666666666666e0_f64 * t20278 - 0.16431333333333333333e0_f64 * t20280 + 0.19931111111111111111e0_f64 * t20283 - 0.59793333333333333333e0_f64 * t20285 - 0.29896666666666666667e0_f64 * t20287 + 0.33218518518518518518e0_f64 * t24230 - 0.11958666666666666667e1_f64 * t24234;
    (t24267, t24271, t24272, t24274, t24275, t24285)
}
