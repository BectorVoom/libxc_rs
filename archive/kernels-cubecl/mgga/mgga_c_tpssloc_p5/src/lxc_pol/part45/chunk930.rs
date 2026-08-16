//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 930/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk930<F: Float>(t2679: F, t6605: F, t6612: F, t23094: F, t30703: F, t23097: F, t23098: F, t23103: F, t794: F, t8339: F, t2684: F, t30719: F, t808: F, t8344: F) -> (F, F, F, F, F, F) {
    let t112832 = t6605 * t6612 * t2679;
    let t112834 = t23094 * t30703;
    let t112837 = t23097 * t6612 * t23098;
    let t112840 = t23103 * t794 * t8339;
    let t112843 = t6605 * t6612 * t2684;
    let t112846 = t808 * t30719 * t8344;
    (t112832, t112834, t112837, t112840, t112843, t112846)
}
