//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 821/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk821<F: Float>(t10151: F, t2464: F, t2465: F, t2487: F, t10417: F, t1415: F, t7030: F, t12960: F, t31051: F, t41588: F, t41592: F, t41595: F, t41600: F, t41604: F, t41607: F, t41610: F, t41613: F, t41616: F, t41619: F, t41621: F, t41624: F, t41627: F, t41630: F, t41631: F, t41636: F) -> F {
    let t41640 = t2487 * t2464 * t2465 * t10151;
    let t41643 = t1415 * t10417 * t7030;
    let t41645 = t31051 * t12960;
    let t41646 = F::new(0.19171462976960374838e1) * t41645;
    let t41647 = F::new(0.19171462976960374838e1) * t41588 - F::new(0.11502877786176224903e1) * t41592 - t41595 + t41600 - t41604 - t41607 - t41610 + t41613 + t41616 - t41619 + F::new(0.59584149919750711116e-1) * t41621 + t41624 + t41627 + t41630 + F::new(0.38342925953920749676e0) * t41631 + F::new(0.38342925953920749676e0) * t41636 - F::new(0.85206502119823888169e-1) * t41640 - F::new(0.29792074959875355558e-1) * t41643 + t41646;
    t41647
}
