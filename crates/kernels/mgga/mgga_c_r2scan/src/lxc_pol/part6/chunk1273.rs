//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1273/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1273<F: Float>(t23909: F, t2271: F, t7104: F, t23835: F, t23837: F, t23895: F, t23896: F, t23897: F, t23900: F, t23902: F, t23904: F, t23906: F, t23907: F, t1416: F, t2452: F, t4885: F, t899: F) -> (F, F, F, F) {
    let t23910 = 0.35089341735807877242e1 * t23909;
    let t23911 = t2271 * t7104;
    let t23913 = -t23835 - t23837 - t23895 - t23896 + t23897 - t23900 - t23902 - t23904 + t23906 - 0.14178e2 * t23907 - t23910 - 0.7089e1 * t23911;
    let t23915 = t1416 * t2452;
    let t23916 = 60.0 * t23915;
    let t23917 = t4885 * t899;
    (t23910, t23913, t23916, t23917)
}
