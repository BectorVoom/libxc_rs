//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 840/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk840<F: Float>(t1246: F, t29719: F, t29708: F, t3612: F, t2147: F, t6238: F, t462: F, t1409: F, t1734: F, t7376: F, t24851: F, t1653: F, t27460: F) -> (F, F, F, F, F) {
    let t29720 = t29719 * t1246;
    let t29723 = t29708 * t3612;
    let t29726 = t2147 * t6238;
    let t29727 = t462 * t29726;
    let t29734 = t1409 * t1734;
    let t29735 = t29734 * t7376;
    let t29736 = t24851 * t29735;
    let t29740 = t27460 * t1653;
    (t29720, t29723, t29727, t29736, t29740)
}
