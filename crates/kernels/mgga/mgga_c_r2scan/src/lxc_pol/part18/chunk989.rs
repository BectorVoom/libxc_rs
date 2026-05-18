//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 989/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk989<F: Float>(t11577: F, t11609: F, t11614: F, t11860: F, t354: F, t1039: F, t3461: F, t1010: F, t11033: F, t11036: F, t2381: F, t2391: F, t3358: F) -> (F, F, F, F, F, F) {
    let t11862 = t11577 + t11609 + t11614 + t11860;
    let t11863 = t354 * t11862;
    let t11864 = t1039 * t3461;
    let t11866 = t11033 * t1010;
    let t11868 = t11036 * t2381;
    let t11870 = t3358 * t2391;
    (t11862, t11863, t11864, t11866, t11868, t11870)
}
