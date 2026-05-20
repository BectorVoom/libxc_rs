//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1638/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1638<F: Float>(t4147: F, t6781: F, t4140: F, t6836: F, t1353: F, t13615: F, t13620: F, t13623: F, t13634: F, t13635: F, t22187: F, t22189: F, t22192: F, t22194: F, t22196: F, t22197: F, t22198: F, t22199: F, t22200: F, t22201: F, t22202: F, t4139: F, t5536: F, t9394: F, t9415: F) -> F {
    let t22466 = t6781 * t4147;
    let t22470 = t4140 * t6836;
    let t22473 = -F::new(3.0) * t1353 * t22466 * t4139 + F::new(6.0) * t22470 * t5536 - t13615 - t13620 - t13623 + t13634 - t13635 - t22187 + t22189 - t22192 + t22194 + t22196 - t22197 - t22198 - t22199 - t22200 + t22201 + t22202 + t9394 - t9415;
    t22473
}
