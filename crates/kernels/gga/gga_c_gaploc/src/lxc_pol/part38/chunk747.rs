//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 747/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk747<F: Float>(t31747: F, t6508: F, t2787: F, t6509: F, t123: F, t25760: F, t1352: F, t3339: F, t10281: F, t501: F, t1853: F, t3432: F) -> (F, F, F, F, F, F) {
    let t31748 = t6508 * t31747;
    let t31769 = t2787 * t6509;
    let t31903 = t25760 * t123;
    let t32067 = t3339 * t1352;
    let t32100 = t10281 * t501;
    let t32112 = t3432 * t1853;
    (t31748, t31769, t31903, t32067, t32100, t32112)
}
