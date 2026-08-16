//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2011/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2011<F: Float>(t22813: F, t6589: F, t80782: F, t23124: F, t23138: F, t6604: F, t6606: F, t22690: F, t2627: F, t10024: F, t1899: F, t2693: F, t6609: F) -> (F, F, F, F, F, F, F) {
    let t81902 = t22813 * t6589 * t80782;
    let t81903 = t81902 * t23124;
    let t81911 = t23138 * t6604;
    let t81912 = t81911 * t6606;
    let t81914 = t22690 * t2627;
    let t81920 = t1899 * t10024;
    let t81921 = F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t81920;
    let t81928 = t6609 * t2693;
    (t81902, t81903, t81911, t81912, t81914, t81921, t81928)
}
