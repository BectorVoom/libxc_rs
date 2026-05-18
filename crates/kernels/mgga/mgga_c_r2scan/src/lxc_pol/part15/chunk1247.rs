//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1247/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1247<F: Float>(t322: F, t40892: F, t40923: F, t40954: F, t40986: F, t41019: F, t41054: F, t41086: F, t2449: F, t3461: F, t374: F, t40505: F, t40509: F, t40526: F, t40528: F, t40532: F, t40536: F, t40539: F, t40541: F, t40544: F, t40547: F, t40551: F, t40554: F, t40569: F, t40571: F, t40578: F) -> F {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t41088 = piecewise5::<f64>(t323, t40892, t331, t40923 + t40954 + t40986 + t41019, t41054 + t41086);
    let t41090 = F::new(2.0) * t2449 * t3461 + t374 * t41088 + t40505 - t40509 - t40526 - t40528 - t40532 - t40536 - t40539 + t40541 + t40544 + t40547 + t40551 - t40554 + t40569 + t40571 - t40578;
    t41090
}
