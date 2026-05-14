//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 33/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk33<F: Float>(t12: F, t18: F, t26: F, t15: F) -> (F, F, F, F, F, F) {
    let t65 = 0.1549425e1 * t12;
    let t66 = 0.420775e0 * t18;
    let t67 = 0.1562925e0 * t26;
    let t68 = 0.705945e1 * t15 + t65 + t66 + t67;
    let t71 = 1.0 + 0.32163958997385070134e2 / t68;
    let t72 = f64::ln(t71);
    (t65, t66, t67, t68, t71, t72)
}
