//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 696/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk696<F: Float>(t10122: F, t874: F, t1445: F, t574: F, t2877: F, t3149: F, t3153: F, t10497: F, t895: F, t10340: F, t1562: F, t2854: F, t3116: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12904 = t10122 * t874;
    let t12905 = t1445 * t12904;
    let t12906 = t574 * t12905;
    let t12909 = F::new(0.35750489951850426669e0) * t3149 * t2877;
    let t12911 = F::new(0.35750489951850426669e0) * t3153 * t2877;
    let t12912 = t895 * t10497;
    let t12914 = t10340 * t874;
    let t12915 = t1445 * t12914;
    let t12916 = t1562 * t12915;
    let t12918 = t2854 * t3116;
    (t12904, t12905, t12906, t12909, t12911, t12912, t12914, t12915, t12916, t12918)
}
