//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1310/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1310(t33538: f64, t10782: f64, t1391: f64, t1392: f64, t1890: f64, t2684: f64, t32260: f64, t32261: f64, t33493: f64, t33495: f64, t33497: f64, t33501: f64, t33505: f64, t33508: f64, t33518: f64, t33522: f64, t33526: f64, t33529: f64, t33532: f64, t33536: f64, t4820: f64, t5598: f64, t5840: f64, t590: f64) -> f64 {
    let t33539 = 0.9585731488480187419e0_f64 * t33538;
    let t33540 = -0.79445533226334281486e-1_f64 * t5598 * t4820 * t32261 + t33493 - t33495 - t33497 - t33501 - t33505 + t33508 + 0.11360866949309851756e0_f64 * t2684 * t1391 * t1392 * t10782 + 0.1022478025437886658e1_f64 * t5840 * t1890 * t32260 * t590 + t33518 + t33522 - t33526 - t33529 - t33532 + t33536 + t33539;
    t33540
}
