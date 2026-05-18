//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1398/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1398<F: Float>(t34719: F, t1540: F, t196: F, t20157: F, t3176: F, t4525: F, t8124: F, t1560: F, t31775: F, t199: F, t31764: F, t31770: F, t595: F) -> (F, F, F, F, F) {
    let t34720 = F::new(0.19171462976960374838e0) * t34719;
    let t34726 = F::new(0.12269736305254639897e2) * t196 * t4525 * t20157 * t8124 * t3176 * t1540;
    let t34730 = F::new(0.27606906686822939768e2) * t196 * t1560 * t20157 * t31775;
    let t34733 = F::new(0.40899121017515466321e1) * t199 * t20157 * t31764;
    let t34737 = F::new(0.19427082483319846503e2) * t196 * t595 * t20157 * t31770;
    (t34720, t34726, t34730, t34733, t34737)
}
