//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1115/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1115<F: Float>(t2295: F, t3113: F, t1201: F, t6121: F, t1196: F, t2279: F, t6313: F, t3135: F, t6233: F, t22233: F, t1208: F, t2318: F) -> (F, F, F, F, F, F, F) {
    let t22564 = t3113 * t2295;
    let t22567 = t1201 * t6121;
    let t22575 = t2279 * t1196;
    let t22639 = t6313 * t1196;
    let t22662 = t3135 * t6233;
    let t22693 = F::new(0.37083333333333333334e-1) * t22233;
    let t22699 = t2318 * t1208;
    (t22564, t22567, t22575, t22639, t22662, t22693, t22699)
}
