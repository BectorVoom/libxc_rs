//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1358/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1358<F: Float>(t1267: F, t26975: F, t5329: F, t5341: F, t11081: F, t26960: F, t28106: F, t1856: F, t3616: F, t7772: F, t96727: F, t1851: F, t26996: F) -> (F, F, F, F, F) {
    let t97039 = t5329 * t26975 * t5341 * t1267;
    let t97051 = F::cast_from(0.7722800925925925926e-4_f64) * t26960 * t11081 * t28106;
    let t97056 = t5329 * t26975 * t1856 * t3616;
    let t97060 = F::cast_from(0.92754700520833333333e-4_f64) * t7772 * t96727;
    let t97063 = t5329 * t26996 * t1851 * t3616;
    (t97039, t97051, t97056, t97060, t97063)
}
