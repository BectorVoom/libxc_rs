//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3128/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3128(t5245: f64, t6628: f64, t20816: f64, t5293: f64, t12855: f64, t17505: f64, t17729: f64, t20317: f64, t21037: f64, t21184: f64, t21242: f64, t24836: f64, t3367: f64, t3604: f64, t3626: f64, t3720: f64, t4181: f64, t44484: f64, t44500: f64, t44502: f64, t45371: f64, t5270: f64, t5297: f64, t5348: f64, t5352: f64, t6587: f64, t70303: f64, t70800: f64, t82293: f64) -> (f64, f64) {
    let t82321 = t5245 * t6628;
    let t82338 = t5293 * t20816;
    let t82340 = 0.85748036236139473944e-3_f64 * t17729 * t3626 * t6587 * t3367 * t4181 - 0.64311027177104605458e-3_f64 * t70800 * t5348 - 0.12862205435420921092e-2_f64 * t44500 * t3720 * t82293 * t44502 + 0.17149607247227894789e-2_f64 * t70303 * t21037 - 0.12862205435420921092e-2_f64 * t44484 * t24836 - 0.12862205435420921092e-2_f64 * t12855 * t3720 * t82321 * t3604 + 0.85748036236139473944e-3_f64 * t17729 * t3626 * t20317 * t5297 - 0.21437009059034868486e-3_f64 * t45371 * t3720 * t82293 * t5352 + 0.91464571985215438872e-2_f64 * t21242 * t5270 - 0.22866142996303859718e-2_f64 * t17505 * t21184 - 0.22866142996303859718e-2_f64 * t82338;
    (t82321, t82340)
}
