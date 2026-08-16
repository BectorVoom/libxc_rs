//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3128/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3128<F: Float>(t5245: F, t6628: F, t20816: F, t5293: F, t12855: F, t17505: F, t17729: F, t20317: F, t21037: F, t21184: F, t21242: F, t24836: F, t3367: F, t3604: F, t3626: F, t3720: F, t4181: F, t44484: F, t44500: F, t44502: F, t45371: F, t5270: F, t5297: F, t5348: F, t5352: F, t6587: F, t70303: F, t70800: F, t82293: F) -> (F, F) {
    let t82321 = t5245 * t6628;
    let t82338 = t5293 * t20816;
    let t82340 = F::cast_from(0.85748036236139473944e-3_f64) * t17729 * t3626 * t6587 * t3367 * t4181 - F::cast_from(0.64311027177104605458e-3_f64) * t70800 * t5348 - F::cast_from(0.12862205435420921092e-2_f64) * t44500 * t3720 * t82293 * t44502 + F::cast_from(0.17149607247227894789e-2_f64) * t70303 * t21037 - F::cast_from(0.12862205435420921092e-2_f64) * t44484 * t24836 - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t3720 * t82321 * t3604 + F::cast_from(0.85748036236139473944e-3_f64) * t17729 * t3626 * t20317 * t5297 - F::cast_from(0.21437009059034868486e-3_f64) * t45371 * t3720 * t82293 * t5352 + F::cast_from(0.91464571985215438872e-2_f64) * t21242 * t5270 - F::cast_from(0.22866142996303859718e-2_f64) * t17505 * t21184 - F::cast_from(0.22866142996303859718e-2_f64) * t82338;
    (t82321, t82340)
}
