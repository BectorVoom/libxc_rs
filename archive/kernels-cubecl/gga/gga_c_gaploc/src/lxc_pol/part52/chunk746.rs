//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 746/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk746<F: Float>(t31585: F, t6508: F, t2293: F, t986: F, t2787: F, t6509: F, t123: F, t25760: F, t2925: F, t935: F, t7290: F, t1022: F, t2530: F) -> (F, F, F, F, F, F, F) {
    let t31586 = t6508 * t31585;
    let t31590 = t986 * t2293;
    let t31591 = t6508 * t31590;
    let t31769 = t2787 * t6509;
    let t31903 = t25760 * t123;
    let t32356 = t2925 * t935;
    let t32357 = t7290 * t32356;
    let t32435 = t1022 * t2530;
    (t31586, t31591, t31769, t31903, t32356, t32357, t32435)
}
