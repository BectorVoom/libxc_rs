//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 776/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk776<F: Float>(t2325: F, t29661: F, t882: F, t883: F, t2326: F, t9074: F, t9079: F, t2317: F, t6525: F, t9066: F, t1365: F, t30209: F) -> (F, F, F, F) {
    let t39798 = t882 * t2325 * t883 * t29661;
    let t39805 = t9074 * t9079 * t2326;
    let t39808 = t6525 * t9066 * t2317;
    let t39811 = t6525 * t1365 * t30209;
    (t39798, t39805, t39808, t39811)
}
