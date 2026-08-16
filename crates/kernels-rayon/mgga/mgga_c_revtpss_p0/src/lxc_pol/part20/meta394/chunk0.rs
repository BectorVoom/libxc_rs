//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1449/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1449(t3006: f64, t2986: f64, t973: f64, t981: f64, t11501: f64, t3011: f64, t4733: f64, t3014: f64, t1100: f64, t11108: f64, t12190: f64, t3329: f64, t3333: f64, t3336: f64, t41229: f64, t41241: f64, t41243: f64, t41449: f64, t41451: f64, t41453: f64, t41455: f64, t41459: f64, t5023: f64) -> (f64, f64, f64, f64, f64) {
    let t41464 = t3006 * t3006;
    let t41468 = 0.35089341735807877242e1_f64 * t981 * t2986 * t41464 * t973;
    let t41472 = 0.69263436422725855036e2_f64 * t981 * t3011 * t11501 * t4733;
    let t41476 = 0.51947577317044391277e2_f64 * t981 * t3011 * t41464 * t3014;
    let t41477 = -4.0_f64 * t1100 * t12190 * t3336 * t5023 + 12.0_f64 * t11108 * t3329 * t3333 * t5023 + t41229 - t41241 - t41243 - t41449 + t41451 - t41453 - t41455 + t41459 + t41468 - t41472 - t41476;
    (t41464, t41468, t41472, t41476, t41477)
}
