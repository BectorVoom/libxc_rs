//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1056/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1056<F: Float>(t10288: F, t10297: F, t10300: F, t10403: F, t4791: F, t4794: F, t4798: F, t4806: F, t4979: F, t4981: F, t4984: F, t4992: F, t4996: F, t6966: F, t881: F, t9797: F, t9906: F, t9907: F, t9908: F) -> (F,) {
    let t10568 = -t9906 - t9907 + t4979 + t4981 + t4984 + 3.0 * t6966 + 3.0 * t9797 - t4791 + t4794 + t4798 - t4806 - 0.7089e1 * t881 * t10288 - 0.2363e1 * t881 * t10297 - 0.7089e1 * t881 * t10300 + t10403 + t4992 - t9908 - t4996;
    (t10568,)
}
