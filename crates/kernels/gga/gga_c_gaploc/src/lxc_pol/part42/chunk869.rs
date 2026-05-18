//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 869/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk869<F: Float>(t2684: F, t2685: F, t45466: F, t11608: F, t2464: F, t2465: F, t2365: F, t35550: F, t7630: F, t13635: F, t23157: F, t11844: F, t2021: F, t7372: F) -> (F, F, F, F, F) {
    let t45468 = t2684 * t2685 * t45466;
    let t45469 = F::new(0.19171462976960374838e0) * t45468;
    let t45472 = t2684 * t2464 * t2465 * t11608;
    let t45473 = F::new(0.42603251059911944084e-1) * t45472;
    let t45475 = t7630 * t2365 * t35550;
    let t45476 = F::new(0.29792074959875355558e-1) * t45475;
    let t45513 = t23157 * t13635;
    let t45516 = t2021 * t11844 * t7372;
    (t45469, t45473, t45476, t45513, t45516)
}
