//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1449/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1449<F: Float>(t3006: F, t2986: F, t973: F, t981: F, t11501: F, t3011: F, t4733: F, t3014: F, t1100: F, t11108: F, t12190: F, t3329: F, t3333: F, t3336: F, t41229: F, t41241: F, t41243: F, t41449: F, t41451: F, t41453: F, t41455: F, t41459: F, t5023: F) -> (F, F, F, F, F) {
    let t41464 = t3006 * t3006;
    let t41468 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t2986 * t41464 * t973;
    let t41472 = F::cast_from(0.69263436422725855036e2_f64) * t981 * t3011 * t11501 * t4733;
    let t41476 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t3011 * t41464 * t3014;
    let t41477 = -F::new(4.0) * t1100 * t12190 * t3336 * t5023 + F::new(12.0) * t11108 * t3329 * t3333 * t5023 + t41229 - t41241 - t41243 - t41449 + t41451 - t41453 - t41455 + t41459 + t41468 - t41472 - t41476;
    (t41464, t41468, t41472, t41476, t41477)
}
