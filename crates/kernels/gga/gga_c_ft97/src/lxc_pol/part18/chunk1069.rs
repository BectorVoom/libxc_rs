//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1069/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1069<F: Float>(t25838: F, t428: F, t3099: F, t398: F, t51: F, t6: F, t370: F, t971: F, t8216: F, t986: F, t1587: F, t1852: F, t10969: F, t463: F, t15564: F, t15565: F, t2247: F) -> (F, F, F, F, F, F, F, F) {
    let t58882 = t25838 * t428;
    let t58948 = t3099 * t6 * t51 * t398;
    let t59631 = t370 * t971;
    let t60243 = t8216 * t986;
    let t60426 = t1587 * t1852;
    let t60711 = t463 * t10969;
    let t60901 = t1587 * t971;
    let t61123 = t15564 * t15565 * t2247;
    (t58882, t58948, t59631, t60243, t60426, t60711, t60901, t61123)
}
