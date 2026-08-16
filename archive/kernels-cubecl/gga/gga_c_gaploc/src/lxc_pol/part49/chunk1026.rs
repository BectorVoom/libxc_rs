//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1026/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1026<F: Float>(t32809: F, t32810: F, t43494: F, t1: F, t10083: F, t1022: F, t2084: F, t787: F, t42944: F, t701: F) -> (F, F, F) {
    let t43592 = F::cast_from(0.85801175884441024004e1_f64) * t32809 * t32810 * t43494;
    let t43597 = F::cast_from(0.21450293971110256001e2_f64) * t787 * t2084 * t1022 * t1 * t10083;
    let t43598 = t42944 * t701;
    (t43592, t43597, t43598)
}
