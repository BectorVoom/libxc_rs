//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1208/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1208<F: Float>(t1880: F, t32875: F, t25: F, t7540: F, t28: F, t3701: F, t7752: F, t4028: F, t8326: F, t7676: F, t1458: F, t576: F) -> (F, F, F, F, F, F, F) {
    let t32877 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t32875;
    let t32899 = t25 * t7540;
    let t33065 = t28 * t7540;
    let t33136 = t3701 * t7752;
    let t33151 = t4028 * t8326;
    let t33152 = F::cast_from(2.0_f64) * t33151;
    let t33153 = t7676 * t8326;
    let t33154 = F::cast_from(2.0_f64) * t33153;
    let t33185 = t576 * t1458;
    (t32877, t32899, t33065, t33136, t33152, t33154, t33185)
}
