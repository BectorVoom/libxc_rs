//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 772/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk772<F: Float>(t10628: F, t2365: F, t6111: F, t10893: F, t959: F, t12709: F, t10677: F, t935: F, t1445: F, t813: F, t2949: F, t3234: F) -> (F, F, F, F, F, F, F, F) {
    let t13118 = t2365 * t10628;
    let t13119 = t6111 * t13118;
    let t13120 = F::new(0.59584149919750711116e-1) * t13119;
    let t13121 = t10893 * t959;
    let t13124 = F::new(0.19171462976960374838e1) * t12709;
    let t13125 = t10677 * t935;
    let t13126 = t1445 * t13125;
    let t13127 = t813 * t13126;
    let t13129 = t2949 * t3234;
    (t13118, t13120, t13121, t13124, t13125, t13126, t13127, t13129)
}
