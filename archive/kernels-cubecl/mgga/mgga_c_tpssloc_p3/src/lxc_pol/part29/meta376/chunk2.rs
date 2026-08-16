//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1506/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1506<F: Float>(t13654: F, t913: F, t893: F, t2929: F, t4471: F, t4497: F, t959: F, t2904: F, t952: F, t3216: F, t4696: F, t13550: F) -> (F, F, F, F, F) {
    let t13655 = t13654 * t913;
    let t13657 = F::cast_from(1.0_f64) * t893 * t13655;
    let t13658 = t2929 * t4471;
    let t13659 = t13658 * t4497;
    let t13661 = F::cast_from(0.34631718211362927518e2_f64) * t959 * t13659;
    let t13662 = t2904 * t4471;
    let t13663 = t13662 * t952;
    let t13665 = F::cast_from(0.23392894490538584828e1_f64) * t959 * t13663;
    let t13666 = t4696 * t3216;
    let t13675 = F::cast_from(0.22076e0_f64) * t13550;
    (t13657, t13661, t13665, t13666, t13675)
}
