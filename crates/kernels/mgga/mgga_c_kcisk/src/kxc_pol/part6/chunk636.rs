//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 636/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk636<F: Float>(t1964: F, t5396: F, t755: F, t5399: F, t763: F, t10690: F, t591: F, t10696: F, t10463: F, t786: F, t10487: F, t1849: F, t2020: F, t10791: F, t397: F, t782: F) -> (F, F, F, F, F, F, F, F) {
    let t12058 = 1.0 / t5396 / t1964;
    let t12059 = t755 * t12058;
    let t12061 = 1.0 / t5399 / t763;
    let t12098 = t591 * t10690;
    let t12105 = t591 * t10696;
    let t12169 = t786 * t10463;
    let t12198 = t786 * t10487;
    let t12234 = t2020 * t1849;
    let t12246 = t397 * t10791 * t786;
    let t12248 = 0.9994882620098509563e-2 * t782 * t12246;
    (t12059, t12061, t12098, t12105, t12169, t12198, t12234, t12248)
}
