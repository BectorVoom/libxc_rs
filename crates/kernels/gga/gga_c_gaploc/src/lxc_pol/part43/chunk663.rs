//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 663/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk663<F: Float>(t10012: F, t7291: F, t2089: F, t588: F, t10007: F, t7068: F, t1984: F, t9804: F, t2012: F, t7426: F, t10929: F, t10912: F, t1422: F, t787: F, t5558: F, t952: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22634 = t10012 * t7291;
    let t22665 = t588 * t2089;
    let t22980 = t10007 * t7068;
    let t22984 = t10012 * t7068;
    let t23000 = t1984 * t9804;
    let t23157 = t2012 * t7426;
    let t23220 = t1984 * t10929;
    let t23477 = t787 * t10912 * t1422;
    let t23555 = t952 * t5558;
    (t22634, t22665, t22980, t22984, t23000, t23157, t23220, t23477, t23555)
}
