//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 814/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk814<F: Float>(t11112: F, t2617: F, t7810: F, t2365: F, t33087: F, t8775: F, t10639: F, t10912: F, t787: F, t899: F, t913: F, t33206: F, t959: F) -> (F, F, F, F) {
    let t43442 = t7810 * t11112 * t2617;
    let t43446 = t8775 * t2365 * t33087;
    let t43454 = t787 * t10912 * t899 * t913 * t10639;
    let t43458 = t33206 * t959;
    (t43442, t43446, t43454, t43458)
}
