//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1091/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1091<F: Float>(t2887: F, t5616: F, t68: F, t5612: F, t771: F, t178: F, t299: F, t301: F, t4902: F, t5604: F, t775: F, t2065: F, t2082: F) -> (F, F, F, F, F) {
    let t17890 = t2887 * t68 * t5616;
    let t17897 = t771 * t5612;
    let t17902 = F::cast_from(0.14820648238345094262e-3_f64) * t299 * t178 * t4902 * t301;
    let t17903 = t5604 * t775;
    let t17905 = t2082 * t2065;
    (t17890, t17897, t17902, t17903, t17905)
}
