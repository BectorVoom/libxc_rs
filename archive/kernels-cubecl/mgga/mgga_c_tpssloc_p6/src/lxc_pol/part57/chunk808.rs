//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 808/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk808<F: Float>(t1976: F, t5493: F, t1458: F, t7670: F, t19596: F, t2019: F, t1983: F, t7458: F, t7468: F, t1873: F, t6287: F, t652: F) -> (F, F, F, F, F, F, F) {
    let t28852 = t1976 * t5493;
    let t28855 = t7670 * t1458;
    let t28860 = t2019 * t19596;
    let t28861 = t1983 * t28860;
    let t28863 = F::cast_from(4.0_f64) * t7458 * t7468;
    let t28864 = t6287 * t1873;
    let t28866 = F::cast_from(2.0_f64) * t652 * t28864;
    (t28852, t28855, t28860, t28861, t28863, t28864, t28866)
}
