//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1180/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1180<F: Float>(t21430: F, t717: F, t720: F, t21073: F, t21145: F, t3: F, t585: F, t21076: F, t5244: F, t1731: F, t21079: F, t21082: F, t636: F, t12: F, t273: F, t625: F) -> (F, F, F, F, F, F) {
    let t21519 = t717 * t21430 * t720;
    let t21529 = t585 * t21145 * t3 * t21073;
    let t21531 = t5244 * t21076;
    let t21533 = t1731 * t21079;
    let t21535 = t636 * t21082;
    let t21537 = f64::powf(t12, -0.35e1);
    let t21540 = t21537 * t3 * t273 * t625;
    (t21519, t21529, t21531, t21533, t21535, t21540)
}
