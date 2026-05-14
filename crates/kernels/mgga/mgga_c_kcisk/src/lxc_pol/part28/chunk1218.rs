//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1218/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1218<F: Float>(t10012: F, t1636: F, t33219: F, t2804: F, t34484: F, t7552: F, t33225: F) -> (F, F, F, F, F) {
    let t34533 = t10012 * t1636;
    let t34534 = t33219 * t34533;
    let t34537 = t2804 * t34484;
    let t34547 = t7552 * t1636;
    let t34548 = t33225 * t34547;
    (t34533, t34534, t34537, t34547, t34548)
}
