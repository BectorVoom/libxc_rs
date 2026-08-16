//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1088/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1088<F: Float>(t1378: F, t32757: F, t225: F, t567: F, t7722: F, t214: F, t1985: F, t2015: F, t7749: F, t3887: F, t26193: F, t8458: F) -> (F, F, F, F, F, F) {
    let t32758 = t1378 * t32757;
    let t32761 = t7722 * t225 * t567;
    let t32762 = t214 * t32761;
    let t32764 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t32762;
    let t32765 = t2015 * t7749;
    let t32766 = t3887 * t32765;
    let t32769 = t26193 * t8458;
    (t32758, t32761, t32762, t32764, t32766, t32769)
}
