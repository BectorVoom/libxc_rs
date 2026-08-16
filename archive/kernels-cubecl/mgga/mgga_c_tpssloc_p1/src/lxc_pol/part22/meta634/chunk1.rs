//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2171/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2171<F: Float>(t54701: F, t12214: F, t67: F, t792: F, t133: F, t1799: F, t40369: F, t6600: F, t131: F, t205: F, t40024: F, t1336: F, t242: F, t40042: F) -> (F, F, F, F, F) {
    let t54702 = F::cast_from(0.11666666666666666666e0_f64) * t54701;
    let t54718 = t792 * t12214 * t67;
    let t54725 = t40369 * t133 * t6600 * t1799;
    let t54728 = t205 * t40024 * t131;
    let t54744 = t1336 * t40042 * t242;
    (t54702, t54718, t54725, t54728, t54744)
}
