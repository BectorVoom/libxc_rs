//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1166/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1166<F: Float>(t33531: F, t1022: F, t15499: F, t28640: F, t7419: F, t3005: F, t7383: F, t9800: F, t10782: F, t1391: F, t1392: F, t1890: F, t2684: F, t32260: F, t32261: F, t33493: F, t33495: F, t33497: F, t33501: F, t33505: F, t33508: F, t33518: F, t33522: F, t33526: F, t33529: F, t4820: F, t5598: F, t5840: F, t590: F) -> (F,) {
    let t33532 = 0.59584149919750711116e-1 * t33531;
    let t33533 = t15499 * t1022;
    let t33535 = t28640 * t33533 * t7419;
    let t33536 = 0.23005755572352449806e1 * t33535;
    let t33538 = t9800 * t3005 * t7383;
    let t33539 = 0.9585731488480187419e0 * t33538;
    let t33540 = -0.79445533226334281486e-1 * t5598 * t4820 * t32261 + t33493 - t33495 - t33497 - t33501 - t33505 + t33508 + 0.11360866949309851756e0 * t2684 * t1391 * t1392 * t10782 + 0.1022478025437886658e1 * t5840 * t1890 * t32260 * t590 + t33518 + t33522 - t33526 - t33529 - t33532 + t33536 + t33539;
    (t33540,)
}
