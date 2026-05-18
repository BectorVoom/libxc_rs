//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 759/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk759<F: Float>(t2877: F, t3153: F, t10497: F, t895: F, t10340: F, t874: F, t1445: F, t1562: F, t2854: F, t3116: F, t1645: F, t3133: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12911 = F::new(0.35750489951850426669e0) * t3153 * t2877;
    let t12912 = t895 * t10497;
    let t12914 = t10340 * t874;
    let t12915 = t1445 * t12914;
    let t12916 = t1562 * t12915;
    let t12918 = t2854 * t3116;
    let t12919 = t1445 * t12918;
    let t12921 = F::new(0.69017266717057349418e1) * t1562 * t12919;
    let t12922 = t1645 * t3133;
    (t12911, t12912, t12914, t12915, t12916, t12918, t12919, t12921, t12922)
}
